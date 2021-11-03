#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/protobuf.hpp>

namespace indiemotion::net
{
    class MessageTranslator
    {
        MessageTranslator() {}

        indiemotion::protobuf::messages::Message translateProtobuf(std::unique_ptr<Message> message)
        {
            return;
        }
    };
} // namespace indiemotion::net
