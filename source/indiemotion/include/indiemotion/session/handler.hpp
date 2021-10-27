#pragma once
#include <indiemotion/net/handler.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/session/session.hpp>

namespace indiemotion::session
{
    class SessionMessageHandler : public net::MessageHandler_T
    {
    public:
        SessionMessageHandler() {}

        std::optional<std::unique_ptr<net::Message>> handleMessage(std::weak_ptr<Session> sessionPtr,
                                                                   std::unique_ptr<net::Message> messagePtr)
        {
            return {};
        }
    };

} // namespace indiemotion::session
