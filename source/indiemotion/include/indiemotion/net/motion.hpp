#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/motion/mode.hpp>
#include <indiemotion/motion/xform.hpp>

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

    struct UpdateMotionXForm : public net::Payload_T
    {
        motion::MotionXForm xform;

        UpdateMotionXForm(motion::MotionXForm xform) : xform(xform) {}
        UpdateMotionXForm(motion::MotionXForm &&xform) : xform(xform) {}

        PayloadType type() const
        {
            return PayloadType::UpdateMotionXForm;
        }
    };
}