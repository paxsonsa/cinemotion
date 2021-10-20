// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* payload.hpp */
#pragma once

#pragma once
#include <indiemotion/messages.hpp>
#include <indiemotion/motion.hpp>
#include <indiemotion/responses/base/payload.hpp>

namespace indiemotion::responses::motion::curmode
{
    class Payload : public base::Payload
    {
    private:
        indiemotion::motion::ModeValue _m_mode;

    public:
        Payload(indiemotion::motion::ModeValue mode) : _m_mode(mode)
        {
        }

        Kind kind() override
        {
            return Kind::MotionCurrentMode;
        }

        indiemotion::motion::ModeValue mode()
        {
            return _m_mode;
        }
    };
} // namespace indiemotion::messages::init
