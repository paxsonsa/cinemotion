#pragma once

#include <indiemotion/_common.hpp>
#include <indiemotion/device/device.hpp>

namespace indiemotion::messages
{

    struct Message;

    typedef std::function<void(messages::Message)> MessageHandler;

    struct Message
    {

        enum class Kind
        {
            InitSession = 100,

            ClientInitSession = 200
        };

        Kind kind;
    };

    /**
     * @brief The message the client will send to initialize the session.
     * 
     */
    struct ClientInitSessionMsg: public Message
    {   
        // TODO Real Contents
        std::string message = "";

        ClientInitSessionMsg(std::string msg): message(msg) {
            kind = Message::Kind::ClientInitSession;
        }

    };

    struct InitSessionMsg: public Message
    {
        device::DeviceProperties deviceInfo;

        InitSessionMsg(device::DeviceProperties info): deviceInfo(info) {
            kind = Message::Kind::InitSession;
        }
    };


}
