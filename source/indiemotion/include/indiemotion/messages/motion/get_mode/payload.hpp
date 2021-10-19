// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* payload.hpp */
#pragma once
#include <indiemotion/messages/base/payload.hpp>
#include <indiemotion/motion.hpp>
#include <indiemotion/protobuf.hpp>

namespace indiemotion::messages::motion::getmode
{
    class Payload : public base::Payload
    {
    public:
        indiemotion::motion::ModeValue newMode;

        Payload(indiemotion::motion::ModeValue newMode) : newMode(newMode) {}

        Kind kind() override
        {
            return Kind::ListCameras;
        }

        static std::unique_ptr<Payload> create([[maybe_unused]] const protobuf::messages::MotionSetMode rawPayload)
        {
            indiemotion::motion::ModeValue mode;

            switch (rawPayload.mode())
            {
            case indiemotion::protobuf::MotionMode::Off:
                mode = indiemotion::motion::ModeValue::Off;
                break;

            case indiemotion::protobuf::MotionMode::Live:
                mode = indiemotion::motion::ModeValue::Live;
                break;

            case indiemotion::protobuf::MotionMode::Recording:
                mode = indiemotion::motion::ModeValue::Recording;
                break;
            }
            return std::make_unique<Payload>(mode);
        }
    };
}
