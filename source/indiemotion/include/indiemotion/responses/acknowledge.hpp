// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* acknowledge.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/responses/base.hpp>
#include <indiemotion/responses/kind.hpp>

namespace indiemotion::responses::acknowledge
{
    class Response : public base::Response
    {
    public:
        Response(base::ID messageId) : base::Response(messageId) {}

        /**
         * @brief Returns the initsession kind
         * 
         * @return kind 
         */
        Kind kind() override
        {
            return Kind::Acknowledgment;
        }

        bool needsAcknowledgment() override
        {
            // Require a ack when the message is recieved.
            return false;
        }
    };
}