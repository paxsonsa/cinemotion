#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/logging.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/session/server_info.hpp>
#include <indiemotion/session/controller.hpp>

namespace indiemotion {
    const std::string LOGGER_NAME = "com.indiemotion.session.bridge";

    class SessionBridge {
    public:
        SessionBridge(std::shared_ptr<NetMessageDispatcher> dispatcherPtr,
                      std::shared_ptr<SessionController> controller) {
            _m_dispatcher = std::move(dispatcherPtr);
            _logger = logging::get_logger(LOGGER_NAME);
            _m_controller = std::move(controller);

            _m_callback_table[Message::PayloadCase::kInitializeSession] =
                std::bind(&SessionBridge::_process_initialize_session, this, std::placeholders::_1);
            _m_callback_table[Message::PayloadCase::kShutdownSession] =
                std::bind(&SessionBridge::_process_shutdown_session, this, std::placeholders::_1);
//            _m_callback_table[Message::PayloadCase::kGetCameraList] =
//                std::bind(&SessionBridge::_process_get_camera_list, this, std::placeholders::_1);
//
//            _m_callback_table[Message::PayloadCase::kSetActiveCamera] =
//                std::bind(&SessionBridge::_process_set_active_camera, this, std::placeholders::_1);
//            _m_callback_table[Message::PayloadCase::kMotionSetMode] =
//                std::bind(&SessionBridge::_process_motion_set_mode, this, std::placeholders::_1);
//            _m_callback_table[Message::PayloadCase::kMotionGetMode] =
//                std::bind(&SessionBridge::_process_motion_get_mode, this, std::placeholders::_1);
//            _m_callback_table[Message::PayloadCase::kMotionXform] =
//                std::bind(&SessionBridge::_process_motion_xform, this, std::placeholders::_1);
        }

        static const std::string APIVersion;

        [[nodiscard]] static std::string supported_api_version() { return SessionBridge::APIVersion; }

        void process_message(const Message &&message) {

			if (message.payload_case() == Message::PayloadCase::PAYLOAD_NOT_SET) {
				auto exception = BadMessageException("description is missing payload.");
				auto err_message = net_make_error_response_from_exception(message.header().id(), exception);
				_m_dispatcher->dispatch(std::move(err_message));
				return;
			}

            auto potential_callback = _m_callback_table[message.payload_case()];
            if (!potential_callback) {
                auto name = net_get_message_payload_name(message);
                _logger->error("Could not process the description, no callback is registered for payload case: {}",
                               name);
                throw std::runtime_error("no callback specified in table for payload case.");
            }

            auto callback = potential_callback.value();
            try {
                callback(std::move(message));
            } catch (const Exception &err)
            {
                auto err_message = net_make_error_response_from_exception(message.header().id(), err);
                _m_dispatcher->dispatch(std::move(err_message));
                if (err.is_fatal)
                {
                    _logger->error("caught fatal error, shutting down session: {}", err.description);
                    _m_controller->shutdown();
                }
            }
            catch (const std::exception &e)
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
        std::array<std::optional<std::function<void(const Message &&)>>, 128> _m_callback_table;

        void _process_initialize_session(const Message &&message) {

            auto device_info = message.initialize_session().device_info();
            if (device_info.api_version() != supported_api_version())
            {
                _logger->error("API Version is not supported: {}", device_info.api_version());
                throw SessionAPIVersionNotSupportedException();
            }

            _m_controller->initialize();
        }

        void _process_shutdown_session(const Message &&message) {
            _m_controller->shutdown();
        }

//        void _process_get_camera_list(const Message &&description) {
//            auto m = net_make_message_with_response_id(description.header().id());
//            auto payload = m.mutable_camera_list();
//
//            for (auto srcCam: _m_controller->get_cameras()) {
//                auto cam = payload->add_cameras();
//                cam->set_id(srcCam.name);
//            }
//            _m_dispatcher->dispatch(std::move(m));
//        }
//
//        void _process_set_active_camera(const Message &&description) {
//            auto camId = description.set_active_camera().camera_id();
//            _m_controller->set_active_camera(camId);
//        }
//
//        void _process_motion_get_mode(const Message &&description) {
//            auto response = net_make_message_with_response_id(description.header().id());
//            auto payload = response.mutable_motion_active_mode();
//            switch(_m_controller->current_motion_mode())
//            {
//            case (MotionMode::Off):
//            {
//                payload->set_mode(message_payloads::MotionMode::Off);
//                break;
//            }
//            case (MotionMode::Live):
//            {
//                payload->set_mode(message_payloads::MotionMode::Live);
//                break;
//            }
//            case (MotionMode::Recording):
//            {
//                payload->set_mode(message_payloads::MotionMode::Recording);
//                break;
//            }
//            }
//            _m_dispatcher->dispatch(std::move(response));
//        }
//
//        void _process_motion_set_mode(const Message &&description) {
//            auto payload = description.motion_set_mode();
//            switch (payload.mode()) {
//            case message_payloads::MotionMode::Off:_m_controller->set_motion_mode(MotionMode::Off);
//                break;
//            case message_payloads::MotionMode::Live:_m_controller->set_motion_mode(MotionMode::Live);
//                break;
//            case message_payloads::MotionMode::Recording:_m_controller->set_motion_mode(MotionMode::Recording);
//                break;
//            default:break;
//            }
//        }
//
//        void _process_motion_xform(const Message &&description) {
//            if (_m_controller->current_motion_mode() == MotionMode::Off) {
//                return;
//            }
//
//            auto payload = description.motion_xform();
//
//            auto xform = MotionXForm::create(
//                payload.translation().x(),
//                payload.translation().y(),
//                payload.translation().z(),
//                payload.orientation().x(),
//                payload.orientation().y(),
//                payload.orientation().z()
//            );
//            _m_controller->update_motion_xform(std::move(xform));
//        }
    };

    const std::string SessionBridge::APIVersion = "1.0";
}