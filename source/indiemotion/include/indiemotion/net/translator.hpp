#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/logging.hpp>
#include <indiemotion/motion/mode.hpp>
#include <indiemotion/net/acknowledge.hpp>
#include <indiemotion/net/camera.hpp>
#include <indiemotion/net/error.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/motion.hpp>
#include <indiemotion/net/protobuf.hpp>

namespace indiemotion::net
{
    class MessageTranslator
    {
    private:

      std::shared_ptr<spdlog::logger> _logger;

        void populateHeader(protobuf::messages::Header *headerPtr,
                            Identifier msgId,
                            Identifier responseId) const
        {
            headerPtr->set_id((std::string)msgId);
            headerPtr->set_responseid((std::string)responseId);
        }

        void populateHeader(protobuf::messages::Header *headerPtr,
                            Identifier msgId) const
        {
            headerPtr->set_id((std::string)msgId);
        }

        protobuf::messages::Message _makeBaseMessage(const std::unique_ptr<Message> &msg) const
        {
            protobuf::messages::Message m;
            auto headerPtr = m.mutable_header();
            if (msg->inResponseToId()) {
                populateHeader(headerPtr, msg->id(),
                               msg->inResponseToId().value());
            } else {
                populateHeader(headerPtr, msg->id());
            }
            return std::move(m);
        }

    public:
        MessageTranslator()
        {
            _logger = logging::getLogger("com.indiemotion.net.translator");
        }

        indiemotion::protobuf::messages::Message translateMessage(const std::unique_ptr<Message> message) const
        {
            _logger->trace("Translating Message to Protobug: {}", message->id());
            switch (message->payloadType())
            {
            case PayloadType::Acknowledge:
            {
                auto m = _makeBaseMessage(message);
                m.mutable_acknowledge();
                return std::move(m);
            }

            case PayloadType::Error:
            {
                auto payload = message->payloadPtrAs<Error>();

                auto m = _makeBaseMessage(message);
                auto error = m.mutable_error();
                error->set_type(payload->errorType);
                error->set_message(payload->message);

                return std::move(m);
            }

            case PayloadType::GetCameraList:
            {
                throw std::runtime_error("cannot translate PayloadType::GetCameraList");
            }

            case PayloadType::CameraList:
            {
                auto m = _makeBaseMessage(message);
                auto payload = m.mutable_camera_list();
                auto cameraList = message->payloadPtrAs<CameraList>();
                for (auto srcCam : cameraList->cameras)
                {
                    auto cam = payload->add_camera();
                    cam->set_id(srcCam.name);
                }
                return std::move(m);
            }

            case PayloadType::MotionUpdateXForm:
            {
                        }

            case PayloadType::MotionSetMode:
            {
                throw std::runtime_error("cannot translate PayloadType::MotionSetMode");
            }

            case PayloadType::MotionGetMode:
            {
                throw std::runtime_error("cannot translate PayloadType::MotionSetMode");
            }

            case PayloadType::MotionActiveMode:
            {
                auto m = _makeBaseMessage(message);
                auto p = message->payloadPtrAs<MotionActiveMode>();
                auto payload = m.mutable_motion_active_mode();
                switch (p->mode)
                {
                case MotionMode::Off:
                {
                    payload->set_mode(protobuf::payloads::v1::MotionMode::Off);
                    break;
                }
                case MotionMode::Live:
                {
                    payload->set_mode(protobuf::payloads::v1::MotionMode::Live);
                    break;
                }
                case MotionMode::Recording:
                {
                    payload->set_mode(protobuf::payloads::v1::MotionMode::Recording);
                    break;
                }
                }
                return std::move(m);
            }

            case PayloadType::SessionInitilization:
                break;
            case PayloadType::SessionShutdown:
                break;
            case PayloadType::CameraInfo:
                break;

            case PayloadType::Unknown:
            case PayloadType::SetCamera:
                // Not Supported
                break;
            }
            throw std::runtime_error("unsupported message payload type.");
        }

        std::unique_ptr<Message> translateProtobuf(const protobuf::messages::Message protobuf) const
        {
            auto header = protobuf.header();
            switch(protobuf.payload_case())
            {
            case protobuf::messages::Message::kAcknowledge: {
                auto payload =
                    std::make_unique<indiemotion::net::Acknowledge>();
                auto message = makeMessageWithIdAndResponseId(
                    Identifier(header.id()), Identifier(header.responseid()),
                    std::move(payload));
                return std::move(message);
            }
            case protobuf::messages::Message::kError:
            {
                auto inError = protobuf.error();
                auto payload = std::make_unique<Error>(
                    inError.type(),
                    inError.message()
                );
            }
            case protobuf::messages::Message::kGetCameraList: {
                auto payload =
                    std::make_unique<GetCameraList>();
                auto message = makeMessageWithId(Identifier(header.id()),
                                                 std::move(payload));
                return std::move(message);
            }
            case protobuf::messages::Message::kMotionSetMode:
            {

                auto inPayload = protobuf.motion_set_mode();
                auto outPayload =
                    std::make_unique<MotionSetMode>();
                switch(inPayload.mode())
                {
                case protobuf::payloads::v1::MotionMode::Off:
                {
                    outPayload->mode = MotionMode::Off;
                    break;
                }
                case protobuf::payloads::v1::MotionMode::Live:
                {
                    outPayload->mode = MotionMode::Live;
                    break;
                }
                case protobuf::payloads::v1::MotionMode::Recording:
                {
                    outPayload->mode = MotionMode::Recording;
                    break;
                }
                case protobuf::payloads::v1::MotionMode_INT_MIN_SENTINEL_DO_NOT_USE_:
                case protobuf::payloads::v1::MotionMode_INT_MAX_SENTINEL_DO_NOT_USE_:

                    break;
                }

                auto message = makeMessageWithId(Identifier(header.id()),
                                                 std::move(outPayload));
                return std::move(message);
            }
            case protobuf::messages::Message::kMotionGetMode:
            {
                auto payload =
                    std::make_unique<MotionGetMode>();
                auto message = makeMessageWithId(Identifier(header.id()),
                                                 std::move(payload));
                return std::move(message);
            }
            case protobuf::messages::Message::kMotionXform:
            {
                auto inXForm = protobuf.motion_xform();
                auto xform = motion::MotionXForm();
                xform.translation.x = inXForm.translation().x();
                xform.translation.y = inXForm.translation().y();
                xform.translation.z = inXForm.translation().z();
                xform.orientation.x = inXForm.orientation().x();
                xform.orientation.y = inXForm.orientation().y();
                xform.orientation.z = inXForm.orientation().z();

                auto outPayload =
                    std::make_unique<MotionUpdateXForm>(std::move(xform));
                auto message = makeMessageWithId(Identifier(header.id()),
                                                 std::move(outPayload));
                return std::move(message);
            }
            case protobuf::messages::Message::kMotionActiveMode:
            case protobuf::messages::Message::kCameraList:
                throw std::runtime_error("message type is not supported as a client message.");
            case protobuf::messages::Message::PAYLOAD_NOT_SET:
                throw std::runtime_error("malformed message, protobuf payload is not set..");
            }

            return nullptr;
        }
    };
} // namespace indiemotion::net
