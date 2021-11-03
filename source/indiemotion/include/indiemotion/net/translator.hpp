#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/motion/mode.hpp>
#include <indiemotion/net/camera.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/motion.hpp>
#include <indiemotion/net/protobuf.hpp>

namespace indiemotion::net
{
    class MessageTranslator
    {
    private:
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
            if (msg->inResponseToId())
            {
                populateHeader(headerPtr, msg->id(), msg->inResponseToId().value());
            }
            else
            {
                populateHeader(headerPtr, msg->id());
            }
            return std::move(m);
        }

    public:
        MessageTranslator()
        {
        }

        indiemotion::protobuf::messages::Message translateMessage(std::unique_ptr<Message> message) const
        {
            switch (message->payloadType())
            {
            case PayloadType::Acknowledge:
            {
                auto m = _makeBaseMessage(message);
                m.mutable_acknowledge();
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
                case motion::MotionMode::Off:
                {
                    payload->set_mode(protobuf::payloads::v1::MotionMode::Off);
                    break;
                }
                case motion::MotionMode::Live:
                {
                    payload->set_mode(protobuf::payloads::v1::MotionMode::Live);
                    break;
                }
                case motion::MotionMode::Recording:
                {
                    payload->set_mode(protobuf::payloads::v1::MotionMode::Recording);
                    break;
                }
                }
                return std::move(m);
            }
            }
        }
    };
} // namespace indiemotion::net
