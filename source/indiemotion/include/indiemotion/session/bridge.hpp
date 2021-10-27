#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/session/handler.hpp>
#include <indiemotion/session/properties.hpp>
#include <indiemotion/session/session.hpp>

namespace indiemotion::session
{

    class SessionBridge
    {
    private:
        std::shared_ptr<Session> _m_sessionPtr;
        std::shared_ptr<SessionMessageHandler> _m_sessionHandlerPtr;

    public:
        SessionBridge(std::shared_ptr<Session> sessionPtr)
        {
            _m_sessionPtr = sessionPtr;
            _m_sessionHandlerPtr = std::make_shared<SessionMessageHandler>();
        }

        std::unique_ptr<net::Message> initialize()
        {
            // TODO Return Initialize
            auto payload = std::make_unique<SessionProperties>(
                "fakeserver",
                "1.0",
                newFeatureSet(0));
            auto message = net::createMessage(std::move(payload));
            return std::move(message);
        }

        std::optional<std::unique_ptr<net::Message>> processMessage(std::unique_ptr<net::Message> messagePtr) const
        {
            std::optional<std::unique_ptr<net::Message>> response;
            switch (messagePtr->payloadType())
            {
                // TODO
            }
        }
    };
}