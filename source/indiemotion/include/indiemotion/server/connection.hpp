#pragma once

#include <indiemotion/_common.hpp>
#include <indiemotion/messages/messages.hpp>

namespace indiemotion::server
{
    class Connection
    {
    public:
        virtual void bindMessageReciever(const messages::Handler handler) noexcept = 0;
        virtual void send(const indiemotion::messages::BaseMessage message) = 0;
    };
}
