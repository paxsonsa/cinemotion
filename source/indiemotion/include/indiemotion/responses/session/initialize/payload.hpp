// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* init_message.hpp

An initialize message is a server-sent message that begin the
initialization process with the client AFTER the conneciton to the
server is established.
*/
#pragma once
#include <indiemotion/messages.hpp>
#include <indiemotion/responses/base/payload.hpp>
#include <indiemotion/session/properties.hpp>

namespace indiemotion::responses::session::initialize
{
    class Payload : public base::Payload
    {
    private:
        indiemotion::session::Properties _m_sessionProps;

    public:
        Payload(indiemotion::session::Properties properties) : _m_sessionProps(properties)
        {
        }

        Kind kind() override
        {
            return Kind::SessionInit;
        }

        indiemotion::session::Properties properties()
        {
            return _m_sessionProps;
        }
    };
} // namespace indiemotion::messages::init
