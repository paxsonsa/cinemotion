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
            _logger = logging::getLogger(LOGGER_NAME);
            _m_controller = std::move(controller);

            _m_callback_table[NetMessage::PayloadCase::kSessionStart] =
                std::bind(&SessionBridge::_process_session_start, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kSessionShutdown] =
                std::bind(&SessionBridge::_process_session_shutdown, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kGetCameraList] =
                std::bind(&SessionBridge::_process_get_camera_list, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kSetActiveCamera] =
                std::bind(&SessionBridge::_process_set_active_camera, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kMotionSetMode] =
                std::bind(&SessionBridge::_process_motion_set_mode, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kMotionGetMode] =
                std::bind(&SessionBridge::_process_motion_get_mode, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kMotionXform] =
                std::bind(&SessionBridge::_process_motion_xform, this, std::placeholders::_1);
        }

        static const std::string APIVersion;

        [[nodiscard]] static std::string supported_api_version() { return SessionBridge::APIVersion; }

        void process_message(const NetMessage &&message) {
            auto potential_callback = _m_callback_table[message.payload_case()];
            if (!potential_callback) {
                auto name = netGetMessagePayloadName(message);
                _logger->error("Could not process the message, no callback is registered for payload case: {}",
                               name);
                throw std::runtime_error("no callback specified in table for payload case.");
            }

            auto callback = potential_callback.value();
            try {
                callback(std::move(message));
            } catch (const Exception &err)
            {
                auto err_message = netMakeErrorResponseFromException(message.header().id(), err);
                _m_dispatcher->dispatch(std::move(err_message));
                if (err.is_fatal)
                {
                    _logger->error("caught fatal error, shutting down session: {}", err.message);
                    _m_controller->shutdown();
                }
            }
            catch (const std::exception &e)
            {
                _logger->error("unexpected error while processing message: {}", e.what());
                auto exception = UnknownFatalException();
                auto err_message = netMakeErrorResponseFromException(message.header().id(), exception);
                _m_dispatcher->dispatch(std::move(err_message));
                _m_controller->shutdown();
            }
        }

    private:
        std::shared_ptr<spdlog::logger> _logger;
        std::shared_ptr<NetMessageDispatcher> _m_dispatcher;
        std::shared_ptr<SessionController> _m_controller;
        std::array<std::optional<std::function<void(const NetMessage &&)>>, 128> _m_callback_table;

        void _process_session_start(const NetMessage &&message) {

            auto properties = message.session_start().session_properties();
            if (properties.api_version() != supported_api_version())
            {
                _logger->error("API Version is not supported: {}", properties.api_version());
                throw SessionAPIVersionNotSupportedException();
            }

            _m_controller->initialize();
        }

        void _process_session_shutdown(const NetMessage &&message) {
            _m_controller->shutdown();
        }

        void _process_get_camera_list(const NetMessage &&message) {
            auto m = netMakeMessageWithResponseId(message.header().id());
            auto payload = m.mutable_camera_list();

            for (auto srcCam: _m_controller->get_cameras()) {
                auto cam = payload->add_cameras();
                cam->set_id(srcCam.name);
            }
            _m_dispatcher->dispatch(std::move(m));
        }

        void _process_set_active_camera(const NetMessage &&message) {
            auto camId = message.set_active_camera().camera_id();
            _m_controller->set_active_camera(camId);
        }

        void _process_motion_get_mode(const NetMessage &&message) {
            auto response = netMakeMessageWithResponseId(message.header().id());
            auto payload = response.mutable_motion_active_mode();
            switch(_m_controller->current_motion_mode())
            {
            case (MotionMode::Off):
            {
                payload->set_mode(netPayloadsV1::MotionMode::Off);
                break;
            }
            case (MotionMode::Live):
            {
                payload->set_mode(netPayloadsV1::MotionMode::Live);
                break;
            }
            case (MotionMode::Recording):
            {
                payload->set_mode(netPayloadsV1::MotionMode::Recording);
                break;
            }
            }
            _m_dispatcher->dispatch(std::move(response));
        }

        void _process_motion_set_mode(const NetMessage &&message) {
            auto payload = message.motion_set_mode();
            switch (payload.mode()) {
            case netPayloadsV1::MotionMode::Off:_m_controller->set_motion_mode(MotionMode::Off);
                break;
            case netPayloadsV1::MotionMode::Live:_m_controller->set_motion_mode(MotionMode::Live);
                break;
            case netPayloadsV1::MotionMode::Recording:_m_controller->set_motion_mode(MotionMode::Recording);
                break;
            default:break;
            }
        }

        void _process_motion_xform(const NetMessage &&message) {
            if (_m_controller->current_motion_mode() == MotionMode::Off) {
                return;
            }

            auto payload = message.motion_xform();

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
    };

    const std::string SessionBridge::APIVersion = "1.0";
}