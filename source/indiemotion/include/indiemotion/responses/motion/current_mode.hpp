
// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* current_mode.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/motion/mode.hpp>
#include <indiemotion/responses/base.hpp>
#include <indiemotion/responses/kind.hpp>

namespace indiemotion::responses::motion::current_mode
{
    class Response : public base::Response
    {
    private:
        indiemotion::motion::ModeValue _m_mode;

    public:
        Response(base::ID messageId, indiemotion::motion::ModeValue mode) : base::Response(messageId)
        {
            _m_mode = mode;
        }

        /**
         * @brief Returns the initsession kind
         *
         * @return kind
         */
        Kind kind() override
        {
            return Kind::MotionCurrentMode;
        }

        bool needsAcknowledgment() override
        {
            // Require a ack when the message is recieved.
            return false;
        }

        indiemotion::motion::ModeValue mode()
        {
            return _m_mode;
        }
    };
}
