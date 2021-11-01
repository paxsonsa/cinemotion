#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/motion/mode.hpp>

namespace indiemotion::net
{
    struct SetMotionMode : public net::Payload_T
    {
        motion::MotionMode mode;

        SetMotionMode(motion::MotionMode mode) : mode(mode) {}

        PayloadType type() const
        {
            return PayloadType::SetMotionMode;
        }
    };
}