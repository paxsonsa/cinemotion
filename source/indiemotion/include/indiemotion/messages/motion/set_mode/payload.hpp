// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* payload.hpp */
#pragma once
#include <indiemotion/messages/base/payload.hpp>
#include <indiemotion/motion.hpp>
#include <indiemotion/protobuf.hpp>

namespace indiemotion::messages::motion::setmode
{
    class Payload : public base::Payload
    {
    private:
        indiemotion::motion::ModeValue _m_mode;

    public:
        Payload(indiemotion::motion::ModeValue newMode) : _m_mode(newMode) {}

        Kind kind() override
        {
            return Kind::MotionSetMode;
        }

        static std::unique_ptr<Payload> create([[maybe_unused]] const protobuf::messages::MotionSetMode rawPayload)
        {
            indiemotion::motion::ModeValue mode;

            switch (rawPayload.mode())
            {
            case indiemotion::protobuf::messages::MotionMode::Off:
                mode = indiemotion::motion::ModeValue::Off;
                break;

            case indiemotion::protobuf::messages::MotionMode::Live:
                mode = indiemotion::motion::ModeValue::Live;
                break;

            case indiemotion::protobuf::messages::MotionMode::Recording:
                mode = indiemotion::motion::ModeValue::Recording;
                break;
            }
            return std::make_unique<Payload>(mode);
        }

        indiemotion::motion::ModeValue newMode()
        {
            return _m_mode;
        }
    };
}
