#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/motion/mode.hpp>
#include <indiemotion/motion/xform.hpp>

namespace indiemotion::net
{
    struct MotionGetMode : public net::Payload_T
    {
        MotionGetMode() {}

        PayloadType type() const
        {
            return PayloadType::MotionGetMode;
        }
    };

    struct MotionSetMode : public net::Payload_T
    {
        MotionMode mode;

        MotionSetMode(): mode(MotionMode::Off) {}
        MotionSetMode(MotionMode mode) : mode(mode) {}

        PayloadType type() const
        {
            return PayloadType::MotionSetMode;
        }
    };

    struct MotionActiveMode : public net::Payload_T
    {
        MotionMode mode;

        MotionActiveMode(MotionMode mode) : mode(mode) {}

        PayloadType type() const
        {
            return PayloadType::MotionActiveMode;
        }
    };

    struct MotionUpdateXForm : public net::Payload_T
    {
        MotionXForm xform;

        MotionUpdateXForm(const MotionXForm &xform) : xform(xform) {}
        MotionUpdateXForm(MotionXForm &&xform) : xform(std::move(xform)) {}

        PayloadType type() const
        {
            return PayloadType::MotionUpdateXForm;
        }
    };
}