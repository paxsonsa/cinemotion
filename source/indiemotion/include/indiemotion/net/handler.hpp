#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message/message.hpp>
#include <indiemotion/session/session.hpp>

namespace indiemotion::net
{

    class MessageHandler_T
    {
    public:
        virtual ~MessageHandler_T() {}
        virtual std::optional<std::unique_ptr<Message>>
        handleMessage(std::weak_ptr<session::Session> sessionPtr,
                      std::unique_ptr<Message> messagePtr) = 0;
    };
}