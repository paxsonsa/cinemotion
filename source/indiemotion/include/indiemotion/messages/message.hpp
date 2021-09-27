#pragma once
#include <chrono>
#include <type_traits>

#include <indiemotion/_common.hpp>
#include <indiemotion/properties/client.hpp>
#include <indiemotion/properties/session.hpp>
#include <indiemotion/session/features.hpp>

namespace indiemotion::messages
{

    typedef uint32_t UID;

    enum class Kind
    {
        Invalid = -1,

        InitSession = 100,
        AckInitSession = 101

    };

    template <typename T>
    struct get_kind
    {
        static const Kind value = Kind::Invalid;
    };

    struct Message;

    typedef std::function<void(Message)> MessageHandler;

    struct Message
    {
        UID uid;

        Message()
        {   
            using namespace std::chrono;
            uid = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
        }
    };

    /**
     * @brief The message the client will send to initialize the session.
     * 
     */
    struct InitSessionMsg : public Message
    {
        properties::ClientProperties props;

        InitSessionMsg() = default;
        InitSessionMsg(properties::ClientProperties props) : props(props) {}
    };

    struct AckInitSessionMsg : public Message
    {
        properties::SessionProperties props;

        AckInitSessionMsg() = default;
        AckInitSessionMsg(properties::SessionProperties props) : props(props) {}
    };

}
