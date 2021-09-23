#pragma once

namespace indiemotion::messages
{

    struct Message;

    typedef std::function<void(messages::Message)> MessageHandler;

    enum class MessageKind
    {
        InitSession = 32
    };

    struct Message
    {
        MessageKind kind;
    };

}
