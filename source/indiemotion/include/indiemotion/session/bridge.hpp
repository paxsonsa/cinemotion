#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/acknowledge.hpp>
#include <indiemotion/net/camera.hpp>
#include <indiemotion/net/error.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/motion.hpp>
#include <indiemotion/session/handler.hpp>
#include <indiemotion/session/properties.hpp>
#include <indiemotion/session/session.hpp>

namespace indiemotion::session
{
    class SessionBridge
    {
    private:
        std::string _m_name = "indiemotion-default";
        std::shared_ptr<Session> _m_sessionPtr;
        std::unique_ptr<net::AcknowledgeCoordinator> _m_ackCoordinator;

    public:
        static const std::string APIVersion;

        SessionBridge(std::shared_ptr<Session> sessionPtr)
        {
            _m_sessionPtr = sessionPtr;
            _m_ackCoordinator = std::make_unique<net::AcknowledgeCoordinator>();
        }

        SessionBridge(std::string name, std::shared_ptr<Session> sessionPtr)
        {
            _m_name = name;
            _m_sessionPtr = sessionPtr;
            _m_ackCoordinator = std::make_unique<net::AcknowledgeCoordinator>();
        }

        std::string apiVersion() const { return SessionBridge::APIVersion; }

        std::unique_ptr<net::Message> initialize()
        {
            auto payload = std::make_unique<SessionProperties>(
                _m_name,
                apiVersion(),
                newFeatureSet(0));
            _m_sessionPtr->setStatus(Status::Initialized);
            auto message = net::createMessage(std::move(payload));
            message->requiresAcknowledgement(true);

            _m_ackCoordinator->queue(message->id(),
                                     [&]()
                                     { _m_sessionPtr->setStatus(Status::Activated); });

            return std::move(message);
        }

        std::optional<std::unique_ptr<net::Message>> processMessage(std::unique_ptr<net::Message> messagePtr) const
        {
            std::optional<std::unique_ptr<net::Message>> response;
            switch (messagePtr->payloadType())
            {
            case net::PayloadType::Acknowledge:
            {
                if (messagePtr->inResponseToId())
                {
                    _m_ackCoordinator->acknowledge(messagePtr->inResponseToId().value());
                }
                else
                {
                    auto error = std::make_unique<net::Error>(
                        net::ErrorType::InvalidMessage,
                        "acknowledgement message missing id for which message it should acknowledge");
                    auto response = net::createMessage(messagePtr->id(), std::move(error));
                    return response;
                }

                return {};
            }

            // ---------------------------------------------------------------------------------------------------
            // Camera Operations
            case net::PayloadType::GetCameraList:
            {
                auto cameras = _m_sessionPtr->getCameras();
                auto payload = std::make_unique<net::CameraList>(std::move(cameras));
                auto response = net::createMessage(messagePtr->id(), std::move(payload));
                return response;
            }

            case net::PayloadType::SetCamera:
            {
                auto msgPayload = messagePtr->payloadPtrAs<net::SetCamera>();
                _m_sessionPtr->setActiveCamera(msgPayload->cameraId);
                auto camera = _m_sessionPtr->getActiveCamera();
                auto payload = std::make_unique<net::CameraInfo>(camera.value());
                auto response = net::createMessage(messagePtr->id(), std::move(payload));
                return response;
            }

            // ---------------------------------------------------------------------------------------------------
            // Motion Mode Operations
            case net::PayloadType::SetMotionMode:
            {
                auto msgPayload = messagePtr->payloadPtrAs<net::SetMotionMode>();
                _m_sessionPtr->setMotionMode(msgPayload->mode);
                return {};
            }

            // ---------------------------------------------------------------------------------------------------
            // Motion XForm Operations
            case net::PayloadType::UpdateMotionXForm:
            {
                auto msgPayload = messagePtr->payloadPtrAs<net::UpdateMotionXForm>();
                _m_sessionPtr->updateMotionXForm(msgPayload->xform);
                return {};
            }


            // ---------------------------------------------------------------------------------------------------
            // Generic Operations
            case net::PayloadType::Error:           
                // TODO Process Errors
                return {};

            case net::PayloadType::Unknown:
            {
                auto error = std::make_unique<net::Error>(net::ErrorType::CannotProcessMessage,
                                                          "could not process message, unknown payload type.");
                auto response = net::createMessage(messagePtr->id(), std::move(error));
                return response;
            }

            default:
                auto error = std::make_unique<net::Error>(net::ErrorType::CannotProcessMessage,
                                                          "could not process message, handler is not implemented to process contents.");
                auto response = net::createMessage(messagePtr->id(), std::move(error));
                return response;
            }
        }
    };

    const std::string SessionBridge::APIVersion = "1.0";
}