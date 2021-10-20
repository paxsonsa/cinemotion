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
        Payload() {}

        Kind kind() override
        {
            return Kind::MotionGetMode;
        }

        static std::unique_ptr<Payload> create([[maybe_unused]] const protobuf::messages::MotionGetMode rawPayload)
        {
            return std::make_unique<Payload>();
        }
    };
}
