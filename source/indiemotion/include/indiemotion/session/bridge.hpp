#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/logging.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/session/server_info.hpp>
#include <indiemotion/session/session.hpp>
#include <indiemotion/session/property_observer.hpp>

namespace indiemotion
{
	class SessionBridge
	{
	public:
		SessionBridge(std::shared_ptr<NetMessageDispatcher> dispatcherPtr,
			std::shared_ptr<Session> controller)
		{
			_m_dispatcher = std::move(dispatcherPtr);
			_logger = logging::get_logger("bridge");
			_m_controller = std::move(controller);

			_m_callback_table[Message::PayloadCase::kAcknowledge] =
				std::bind(&SessionBridge::_process_acknowledge, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kInitializeSession] =
				std::bind(&SessionBridge::_process_initialize_session, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kShutdownSession] =
				std::bind(&SessionBridge::_process_shutdown_session, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kSessionProperty] =
				std::bind(&SessionBridge::_process_session_property, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kGetSessionPropertyByName] =
				std::bind(&SessionBridge::_process_get_session_property, this, std::placeholders::_1);
            _m_callback_table[Message::PayloadCase::kGetCameraList] =
                std::bind(&SessionBridge::_process_get_camera_list, this, std::placeholders::_1);
			_m_callback_table[Message::PayloadCase::kCameraList] =
				std::bind(&SessionBridge::_process_camera_list, this, std::placeholders::_1);
            _m_callback_table[Message::PayloadCase::kInputDeviceXform] =
                std::bind(&SessionBridge::_process_input_device_xform, this, std::placeholders::_1);
		}

		static const std::string APIVersion;

		[[nodiscard]] static std::string supported_api_version()
		{
			return SessionBridge::APIVersion;
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
		std::shared_ptr<Session> _m_controller;
		std::array<std::optional<std::function<void(const Message&&)>>, 128> _m_callback_table;

		void _send_acknowledgement_for(const Message&& message)
		{
			auto response = net_make_message_with_response_id(message.header().id());
			response.mutable_acknowledge();
			_m_dispatcher->dispatch(std::move(response));
		}

		void _process_acknowledge(const Message&& message) {}

		void _process_initialize_session(const Message&& message)
		{
			auto device_info = message.initialize_session().device_info();
			if (device_info.api_version() != supported_api_version())
			{
				_logger->error("API Version is not supported: {}", device_info.api_version());
				throw APIVersionNotSupportedException();
			}
			_m_controller->initialize();
			_send_acknowledgement_for(std::move(message));

			auto motion_mode = GlobalProperties::MotionCaptureMode();
			_m_controller->get_session_property(&motion_mode);
			auto init_message = net_make_message();
			try {
				_populate_property_message(init_message, std::move(motion_mode));
				_m_dispatcher->dispatch(std::move(init_message));
			} catch (Exception &exception) {
				auto err_message = net_make_error_response_from_exception(message.header().id(), exception);
				_m_dispatcher->dispatch(std::move(err_message));
				return;
			}

			init_message = net_make_message();
			_send_camera_list(std::move(init_message));
		}

		void _process_shutdown_session(const Message&& message)
		{
			_m_controller->shutdown();
			_send_acknowledgement_for(std::move(message));
		}

		void _process_session_property(const Message&& message)
		{
			auto raw_property = message.session_property();
			SessionProperty::Value value;

			switch (raw_property.value_case())
			{
			case indiemotionpb::payloads::SessionProperty::kIntValue:
				value = raw_property.int_value();
				break;
			case indiemotionpb::payloads::SessionProperty::kStringValue:
				value = raw_property.string_value();
				break;
			case indiemotionpb::payloads::SessionProperty::kFloatValue:
				value = raw_property.float_value();
				break;
			case indiemotionpb::payloads::SessionProperty::kBoolValue:
				value = raw_property.bool_value();
				break;
			case indiemotionpb::payloads::SessionProperty::VALUE_NOT_SET:
				auto exception = BadMessageException("session property value type is not set.");
				auto err_message = net_make_error_response_from_exception(message.header().id(), exception);
				_m_dispatcher->dispatch(std::move(err_message));
				return;
			}

			auto property = SessionProperty(raw_property.name(), std::move(value));
			_m_controller->set_session_property(std::move(property));
			_send_acknowledgement_for(std::move(message));
		}

		void _process_get_session_property(const Message&& message)
		{
			auto name = message.get_session_property_by_name().name();
			auto property = SessionProperty(name);

			if (!_m_controller->get_session_property(&property)) {
				auto exception = SessionPropertyNotFoundException(name);
				auto err_message = net_make_error_response_from_exception(message.header().id(), exception);
				_m_dispatcher->dispatch(std::move(err_message));
				return;
			}

			auto response = net_make_message_with_response_id(message.header().id());

			try {
				_populate_property_message(response, std::move(property));
			} catch (Exception &exception) {
				auto err_message = net_make_error_response_from_exception(message.header().id(), exception);
				_m_dispatcher->dispatch(std::move(err_message));
				return;
			}

			_m_dispatcher->dispatch(std::move(response));
		}

		void _process_get_camera_list(const Message&& message)
		{
			auto m = net_make_message_with_response_id(message.header().id());
			_send_camera_list(std::move(m));

		}
		void _send_camera_list(Message&& m)
		{
			auto payload = m.mutable_camera_list();

			for (auto srcCam: _m_controller->get_cameras())
			{
				auto cam = payload->add_cameras();
				cam->set_id(srcCam.name);
			}
			_m_dispatcher->dispatch(std::move(m));
		}

		void _process_camera_list(const Message&& message)
		{
			auto exception = ApplicationException("cannot process camera list updates from input device");
			auto err_message = net_make_error_response_from_exception(message.header().id(), exception);
			_m_dispatcher->dispatch(std::move(err_message));
			return;
		}

        void _process_input_device_xform(const Message &&message) {
            auto payload = message.input_device_xform();
            auto xform = MotionXForm::create(
                payload.translation().x(),
                payload.translation().y(),
                payload.translation().z(),
                payload.orientation().x(),
                payload.orientation().y(),
                payload.orientation().z()
            );
            _m_controller->update_motion_xform(std::move(xform));
        }

		void _populate_property_message(Message &message, SessionProperty &&property)
		{
			auto payload = message.mutable_session_property();
			payload->set_name(property.name());
			auto raw_value = property.value();
			if (property.contains<std::string>())
				payload->set_string_value(std::get<std::string>(*raw_value));
			else if (property.contains<std::int64_t>())
				payload->set_int_value(std::get<std::int64_t>(*raw_value));
			else if (property.contains<double>())
				payload->set_float_value(std::get<double>(*raw_value));
			else if (property.contains<bool>())
				payload->set_bool_value(std::get<double>(*raw_value));
			else
				throw SessionPropertyTypeException("failed to cast type");
		}
	};

	const std::string SessionBridge::APIVersion = "1.0";



}