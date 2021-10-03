// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* init_message.hpp 

An initialize message is a server-sent message that begin the
initialization process with the client AFTER the conneciton to the
server is established.
*/
#pragma once
#include <indiemotion/messages/message.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/session/properties.hpp>

namespace indiemotion::messages
{
    struct InitSessionMessage: public Message
    {
        session::SessionProperties properties;

        InitSessionMessage() = default;
        InitSessionMessage(session::SessionProperties properties): properties(properties) {}

        Kind get_kind()
        {
            return Kind::InitSession;
        }
    };
}
