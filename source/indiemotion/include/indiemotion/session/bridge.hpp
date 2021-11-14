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
                      std::shared_ptr<SessionController> sessionPtr) {
            _m_dispatcher = std::move(dispatcherPtr);
            _logger = logging::getLogger(LOGGER_NAME);
            _m_sessionPtr = std::move(sessionPtr);

            _m_callback_table[NetMessage::PayloadCase::kSessionStart] =
                std::bind(&SessionBridge::_processSessionStart, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kSessionShutdown] =
                std::bind(&SessionBridge::_processSessionShutdown, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kGetCameraList] =
                std::bind(&SessionBridge::_processGetCameraList, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kSetActiveCamera] =
                std::bind(&SessionBridge::_processSetActiveCamera, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kMotionSetMode] =
                std::bind(&SessionBridge::_processMotionSetMode, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kMotionGetMode] =
                std::bind(&SessionBridge::_processMotionGetMode, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kMotionXform] =
                std::bind(&SessionBridge::_processMotionXForm, this, std::placeholders::_1);
        }

        static const std::string APIVersion;

        [[nodiscard]] static std::string supportedAPIVersion() { return SessionBridge::APIVersion; }

        void processMessage(const NetMessage &&message) {
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
                    _m_sessionPtr->shutdown();
                }
            }
            catch (const std::exception &e)
            {
                _logger->error("unexpected error: {}", e.what());
                auto exception = UnknownFatalException();
                auto err_message = netMakeErrorResponseFromException(message.header().id(), exception);
                _m_dispatcher->dispatch(std::move(err_message));
                _m_sessionPtr->shutdown();
            }
        }

    private:
        std::shared_ptr<spdlog::logger> _logger;
        std::shared_ptr<NetMessageDispatcher> _m_dispatcher;
        std::shared_ptr<SessionController> _m_sessionPtr;
        std::array<std::optional<std::function<void(const NetMessage &&)>>, 128> _m_callback_table;

        void _processSessionStart(const NetMessage &&message) {

            auto properties = message.session_start().session_properties();
            if (properties.api_version() != supportedAPIVersion())
            {
                throw SessionAPIVersionNotSupportedException();
            }

            _m_sessionPtr->initialize();
        }

        void _processSessionShutdown(const NetMessage &&message) {
            _m_sessionPtr->shutdown();
        }

        void _processGetCameraList(const NetMessage &&message) {
            auto m = netMakeMessage();
            auto payload = m.mutable_camera_list();

            for (auto srcCam: _m_sessionPtr->getCameras()) {
                auto cam = payload->add_cameras();
                cam->set_id(srcCam.name);
            }
            _m_dispatcher->dispatch(std::move(m));
        }

        void _processSetActiveCamera(const NetMessage &&message) {
            auto camId = message.set_active_camera().camera_id();
            _m_sessionPtr->setActiveCamera(camId);
        }

        void _processMotionGetMode(const NetMessage &&message) {
            auto response = netMakeMessageWithResponseId(message.header().id());
            auto payload = response.mutable_motion_active_mode();
            switch(_m_sessionPtr->currentMotionMode())
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

        void _processMotionSetMode(const NetMessage &&message) {
            auto payload = message.motion_set_mode();
            switch (payload.mode()) {
            case netPayloadsV1::MotionMode::Off:_m_sessionPtr->setMotionMode(MotionMode::Off);
                break;
            case netPayloadsV1::MotionMode::Live:_m_sessionPtr->setMotionMode(MotionMode::Live);
                break;
            case netPayloadsV1::MotionMode::Recording:_m_sessionPtr->setMotionMode(MotionMode::Recording);
                break;
            default:break;
            }
        }

        void _processMotionXForm(const NetMessage &&message) {
            if (_m_sessionPtr->currentMotionMode() == MotionMode::Off) {
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
            _m_sessionPtr->updateMotionXForm(std::move(xform));
        }
    };

    const std::string SessionBridge::APIVersion = "1.0";
}