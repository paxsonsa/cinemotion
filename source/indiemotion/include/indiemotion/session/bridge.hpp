#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/logging.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/dispatcher.hpp>
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

            _m_callback_table[NetMessage::PayloadCase::kSessionActivate] =
                std::bind(&SessionBridge::_processSessionActivate, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kSessionShutdown] =
                std::bind(&SessionBridge::_processSessionShutdown, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kGetCameraList] =
                std::bind(&SessionBridge::_processGetCameraList, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kSetActiveCamera] =
                std::bind(&SessionBridge::_processSetActiveCamera, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kMotionSetMode] =
                std::bind(&SessionBridge::_processMotionSetMode, this, std::placeholders::_1);
            _m_callback_table[NetMessage::PayloadCase::kMotionXform] =
                std::bind(&SessionBridge::_processMotionXForm, this, std::placeholders::_1);

        }

        static const std::string APIVersion;

        [[nodiscard]] static std::string apiVersion() { return SessionBridge::APIVersion; }

        void start() {
            NetMessage message;
            message.mutable_header()->set_id(generateNewIdentifierString());
            auto payload = message.mutable_session_start();
            auto info = payload->mutable_server_info();
            info->set_api_version(apiVersion());
            info->set_features(0);

            _m_sessionPtr->setStatus(SessionStatus::Starting);
            _m_dispatcher->dispatch(std::move(message));
        }

        void processMessage(NetMessage &&message) const {
            // TODO Handle Bad Access
            _m_callback_table[message.payload_case()](std::move(message));
        }

    private:
        std::shared_ptr<spdlog::logger> _logger;
        std::shared_ptr<NetMessageDispatcher> _m_dispatcher;
        std::shared_ptr<SessionController> _m_sessionPtr;

        std::array<std::function<void(NetMessage &&)>, 1024> _m_callback_table;

        void _processSessionActivate(NetMessage &&message) {
            _logger->trace("PayloadCase=SessionActivate");
            _m_sessionPtr->setStatus(SessionStatus::Activated);
        }

        void _processSessionShutdown(NetMessage &&message) {
            _logger->trace("PayloadCase=SessionShutdown");
            _m_sessionPtr->shutdown();
        }

        void _processGetCameraList(NetMessage &&message) {
            _logger->trace("PayloadCase=GetCameraList");

            auto m = netMakeMessage();
            auto payload = m.mutable_camera_list();

            for (auto srcCam: _m_sessionPtr->getCameras()) {
                auto cam = payload->add_cameras();
                cam->set_id(srcCam.name);
            }
            _m_dispatcher->dispatch(std::move(m));
        }

        void _processSetActiveCamera(NetMessage &&message) {
            _logger->trace("PayloadCase=SetActiveCamera");

            auto camId = message.set_active_camera().camera_id();
            _m_sessionPtr->setActiveCamera(camId);
        }

        void _processMotionSetMode(NetMessage &&message) {
            _logger->trace("PayloadCase=MotionSetMode");
            auto payload = message.motion_set_mode();
            switch(payload.mode())
            {
            case netPayloadsV1::MotionMode::Off:
                _m_sessionPtr->setMotionMode(MotionMode::Off);
                break;
            case netPayloadsV1::MotionMode::Live:
                _m_sessionPtr->setMotionMode(MotionMode::Live);
                break;
            case netPayloadsV1::MotionMode::Recording:
                _m_sessionPtr->setMotionMode(MotionMode::Recording);
                break;
            default:
                break;
            }
        }

        void _processMotionXForm(NetMessage &&message) {
            _logger->trace("PayloadCase=MotionXForm");

            if (_m_sessionPtr->currentMotionMode() == MotionMode::Off)
            {
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