#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/protobuf.hpp>

namespace indiemotion::net
{
    class MessageTranslator
    {
    public:
        MessageTranslator() {}

        indiemotion::protobuf::messages::Message translateMessage(std::unique_ptr<Message> message) const
        {
            switch (message->payloadType())
            {
            case PayloadType::Acknowledge:
            {
                protobuf::messages::Message m;
                auto headerPtr = m.mutable_header();
                headerPtr->set_id((std::string)(message->id()));
                headerPtr->set_responseid(
                    (std::string)(message->inResponseToId().value()));

                m.mutable_acknowledge();

                return std::move(m);
            }
            }
        }
    };
} // namespace indiemotion::net
