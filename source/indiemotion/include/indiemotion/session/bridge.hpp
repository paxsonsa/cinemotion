#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/session/session.hpp>

namespace indiemotion::session
{
    class SessionBridge
    {
    private:
        std::shared_ptr<Session> _m_session;

    public:
        SessionBridge(std::shared_ptr<Session> sessionPtr) {}

        std::unique_ptr<net::Message> initialize()
        {
            // TODO Return Initialize
            auto message = net::createMessage(nullptr);
            return std::move(message);
        }

        std::optional<std::unique_ptr<net::Message>> processMessage(std::unique_ptr<net::Message> messagePtr) const
        {
            return {};
        }
    };
}