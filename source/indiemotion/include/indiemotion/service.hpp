#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/logging.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/context.hpp>
#include <indiemotion/delegates.hpp>
#include <indiemotion/services/session_service.hpp>
#include <indiemotion/services/scene_service.hpp>
#include <indiemotion/services/motion_service.hpp>

namespace indiemotion
{
	Message make_message_from(const SceneContext& ctx);
	Message make_message_from(const MotionContext& ctx);

	struct Service
	{
		const std::shared_ptr<Context> ctx;

		Service(std::shared_ptr<NetMessageDispatcher> dispatcher_ptr): ctx(std::make_shared<Context>())
		{
			_dispatcher = std::move(dispatcher_ptr);
			_logger = logging::get_logger("com.indiemotion.sessionservice");

			/// Initialize the callback table
			_m_callback_table[Message::PayloadCase::kInitializeSession] =
				std::bind(&Service::_process_initialize_session, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kSceneInfo] =
				std::bind(&Service::_process_scene_info_update, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kMotionInfo] =
				std::bind(&Service::_process_motion_info_update, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kShutdownSession] =
				std::bind(&Service::_process_shutdown_session, this, std::placeholders::_1);

		}

		[[nodiscard]] static std::string supported_api_version()
		{
			return "1.0";
		}

		void init_session_service(std::shared_ptr<SessionDelegate> delegate)
		{
			_logger->info("initializing session service.");
			_session_service = std::make_shared<SessionService>(ctx, delegate);
			_session_service->initialize();
		}

		void init_scene_service(std::shared_ptr<SceneDelegate> delegate)
		{
			_logger->info("initializing scene service.");
			_scene_service = std::make_shared<SceneService>(ctx, delegate);
			_scene_service->initialize();
		}

		void init_motion_service(std::shared_ptr<MotionDelegate> delegate)
		{
			_logger->info("initializing motion service.");
			_motion_service = std::make_shared<MotionService>(ctx, delegate);
			_motion_service->initialize();
		}

		void process_message(const Message&& message)
		{
			if (message.payload_case() == Message::PayloadCase::PAYLOAD_NOT_SET)
			{
				auto exception = BadMessageException("description is missing payload.");
				auto err_message = net_make_error_response_from_exception(message.header().id(), exception);
				_dispatcher->dispatch(std::move(err_message));
				return;
			}

			auto potential_callback = _m_callback_table[message.payload_case()];
			if (!potential_callback)
			{
				auto name = net_get_message_payload_name(message);
				_logger->error("Could not process the message, no callback is registered for payload case: {}",
					name);
				auto exception = ApplicationException("application does not know how to process message");
				auto err_message = net_make_error_response_from_exception(message.header().id(), exception);
				_dispatcher->dispatch(std::move(err_message));
				return;
			}

			auto callback = potential_callback.value();
			try
			{
				callback(std::move(message));
				return;
			}
			catch (const Exception& err)
			{
				auto err_message = net_make_error_response_from_exception(message.header().id(), err);
				_dispatcher->dispatch(std::move(err_message));
				if (err.is_fatal)
				{
					_logger->error("caught fatal error, shutting down session: {}", err.description);
					_session_service->shutdown();
				}
			}
			catch (const std::exception& e)
			{
				_logger->error("unexpected error while processing description: {}", e.what());
				auto exception = UnknownFatalException();
				auto err_message = net_make_error_response_from_exception(message.header().id(), exception);
				_dispatcher->dispatch(std::move(err_message));
				_session_service->shutdown();
			}
		}

	private:
		std::shared_ptr<spdlog::logger> _logger;
		std::shared_ptr<NetMessageDispatcher> _dispatcher;
		std::shared_ptr<SessionService> _session_service;
		std::shared_ptr<SceneService> _scene_service;
		std::shared_ptr<MotionService> _motion_service;

		std::array<std::optional<std::function<void(const Message&&)>>, 128> _m_callback_table{};

		void _send_current_scene_info()
		{
			auto message = make_message_from(ctx->scene);
			_dispatcher->dispatch(std::move(message));
		}

		void _send_current_motion_info()
		{
			auto message = make_message_from(ctx->motion);
			_dispatcher->dispatch(std::move(message));
		}

		void _process_initialize_session(const Message&& message)
		{
			auto session_info = message.initialize_session().session_info();
			if (session_info.api_version() != supported_api_version())
			{
				_logger->error("API Version is not supported: {}", session_info.api_version());
				throw APIVersionNotSupportedException();
			}
			_session_service->process(session_info);
			_send_current_scene_info();
			_send_current_motion_info();
		}

		void _process_scene_info_update(const Message&& message)
		{
			auto scene_info = message.scene_info();
			_scene_service->process(scene_info);
		}

		void _process_motion_info_update(const Message&& message)
		{
			auto motion_info = message.motion_info();
			_motion_service->process(motion_info);
		}

		void _process_shutdown_session(const Message&& message)
		{
			_session_service->shutdown();
		}
	};

	Message make_message_from(const SceneContext& ctx)
	{
		auto m = net_make_message();
		auto scene_info = m.mutable_scene_info();

		for (auto src_cam: ctx.cameras)
		{
			auto cam = scene_info->add_cameras();
			cam->set_name(src_cam.name);
		}

		if (ctx.active_camera_name.has_value())
		{
			scene_info->set_active_camera_name(ctx.active_camera_name.value());
		}

		return m;
	}

	Message make_message_from(const MotionContext& ctx)
	{
		auto m = net_make_message();
		auto motion_info = m.mutable_motion_info();

		switch(ctx.status)
		{
		case MotionStatus::Idle:
			motion_info->set_status(Payloads::MotionInfo_Status_Idle);
			break;
		case MotionStatus::Live:
			motion_info->set_status(Payloads::MotionInfo_Status_Live);
			break;
		case MotionStatus::Recording:
			motion_info->set_status(Payloads::MotionInfo_Status_Recording);
			break;
		}

		return m;
	}
}