#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/logging.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/controller.hpp>

namespace indiemotion
{
	struct Service
	{
		Service(std::shared_ptr<NetMessageDispatcher> dispatcher_ptr,
			           std::shared_ptr<SessionController> controller)
		{
			_m_dispatcher = std::move(dispatcher_ptr);
			_logger = logging::get_logger("com.indiemotion.sessionservice");
			_m_controller = std::move(controller);

			/// Initialize the callback table
			_m_callback_table[Message::PayloadCase::kInitializeSession] =
				std::bind(&Service::_process_initialize_session, this, std::placeholders::_1);

		}

		[[nodiscard]] static std::string supported_api_version()
		{
			return "1.0";
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
		std::shared_ptr<SessionController> _m_controller;
		std::array<std::optional<std::function<void(const Message&&)>>, 128> _m_callback_table {};

		void _process_initialize_session(const Message&& message)
		{
			auto session_info = message.initialize_session().session_info();
			if (session_info.api_version() != supported_api_version())
			{
				_logger->error("API Version is not supported: {}", session_info.api_version());
				throw APIVersionNotSupportedException();
			}
			_m_controller->initialize(session_info.session_name());
		}
	};
}