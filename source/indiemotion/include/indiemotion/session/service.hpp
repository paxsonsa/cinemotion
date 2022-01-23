#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/logging.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/session/server_info.hpp>
#include <indiemotion/session/session.hpp>

namespace indiemotion
{
	class SessionService
	{
	public:
		SessionService(std::shared_ptr<NetMessageDispatcher> dispatcherPtr,
			std::shared_ptr<SessionCon> controller)
		{
			_m_dispatcher = std::move(dispatcherPtr);
			_logger = logging::get_logger("com.indiemotion.service");
			_m_controller = std::move(controller);

			_m_callback_table[Message::PayloadCase::kAcknowledge] =
				std::bind(&SessionService::_process_acknowledge, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kInitializeSession] =
				std::bind(&SessionService::_process_initialize_session, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kShutdownSession] =
				std::bind(&SessionService::_process_shutdown_session, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kSessionProperty] =
				std::bind(&SessionService::_process_session_property, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kGetSessionPropertyByName] =
				std::bind(&SessionService::_process_get_session_property, this, std::placeholders::_1);
            _m_callback_table[Message::PayloadCase::kGetCameraList] =
                std::bind(&SessionService::_process_get_camera_list, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kCameraList] =
				std::bind(&SessionService::_process_camera_list, this, std::placeholders::_1);
            _m_callback_table[Message::PayloadCase::kInputDeviceXform] =
                std::bind(&SessionService::_process_input_device_xform, this, std::placeholders::_1);
		}

		static const std::string APIVersion;

		[[nodiscard]] static std::string supported_api_version()
		{
			return SessionService::APIVersion;
		}

		void process_message(const Message&& message)
		{
			if (message.payload_case() == Message::PayloadCase::PAYLOAD_NOT_SET)
			{
				auto exception = BadMessageException("description is missing payload.");
				auto err_message = net_make_error_response_from_exception(message.header().id(), exception);
				_m_dispatcher->dispatch(std::move(err_message));
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
				_m_dispatcher->dispatch(std::move(err_message));
				return;
			}

			auto callback = potential_callback.value();
			try
			{
				callback(std::move(message));
			}
			catch (const Exception& err)
			{
				auto err_message = net_make_error_response_from_exception(message.header().id(), err);
				_m_dispatcher->dispatch(std::move(err_message));
				if (err.is_fatal)
				{
					_logger->error("caught fatal error, shutting down session: {}", err.description);
					_m_controller->shutdown();
				}
			}
			catch (const std::exception& e)
			{
				_logger->error("unexpected error while processing description: {}", e.what());
				auto exception = UnknownFatalException();
				auto err_message = net_make_error_response_from_exception(message.header().id(), exception);
				_m_dispatcher->dispatch(std::move(err_message));
				_m_controller->shutdown();
			}
		}

	private:
		std::shared_ptr<spdlog::logger> _logger;
		std::shared_ptr<NetMessageDispatcher> _m_dispatcher;
		std::shared_ptr<SessionCon> _m_controller;
		std::array<std::optional<std::function<void(const Message&&)>>, 128> _m_callback_table;

		void _process_initialize_session(const Message&& message)
		{
			auto device_info = message.initialize_session().device_info();
			if (device_info.api_version() != supported_api_version())
			{
				_logger->error("API Version is not supported: {}", device_info.api_version());
				throw APIVersionNotSupportedException();
			}
			_m_controller->initialize();
		}

		void _process_shutdown_session(const Message&& message)
		{
			_m_controller->shutdown();
			_send_acknowledgement_for(std::move(message));
		}
	};

	const std::string SessionService::APIVersion = "1.0";



}