// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* init_message.hpp 

An initialize message is a server-sent message that begin the
initialization process with the client AFTER the conneciton to the
server is established.
*/
#pragma once
#include <indiemotion/messages/message.hpp>
#include <indiemotion/session/properties.hpp>

namespace indiemotion::messages::init
{
    using namespace messages::response;
    struct InitializeSessionResponse: public Response
    {

        session::Properties properties;

        InitializeSessionResponse() = default;
        InitializeSessionResponse(session::Properties properties): properties(properties) {}

        /**
         * @brief Returns the initsession kind
         * 
         * @return kind 
         */
        response::kind kind() override
        {
            return kind::InitSession;
        }

        bool needsAcknowledgment() override {
            // Require a ack when the message is recieved.
            return true;
        }
    };
} // namespace indiemotion::messages::init
