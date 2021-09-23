#pragma once

#include <indiemotion/_common.hpp>

namespace indiemotion::messages
{

    struct Message;

    typedef std::function<void(messages::Message)> MessageHandler;

    enum class MessageKind
    {
        InitSession = 32,
        InitClientSession = 32
    };

    struct Message
    {
        MessageKind kind;
    };

    struct ClientInitMessage: public Message
    {   
        std::string message = "";

        ClientInitMessage(std::string msg): message(msg) {
            kind = MessageKind::InitClientSession;
        }
    };

}
