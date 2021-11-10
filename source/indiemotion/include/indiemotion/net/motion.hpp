#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/motion/mode.hpp>
#include <indiemotion/motion/xform.hpp>

namespace indiemotion::net
{
    struct MotionGetMode : public NetPayload_T
    {
        MotionGetMode() {}

        NetPayloadType type() const
        {
            return NetPayloadType::MotionGetMode;
        }
    };

    struct MotionSetMode : public NetPayload_T
    {
        MotionMode mode;

        MotionSetMode(): mode(MotionMode::Off) {}
        MotionSetMode(MotionMode mode) : mode(mode) {}

        NetPayloadType type() const
        {
            return NetPayloadType::MotionSetMode;
        }
    };

    struct MotionActiveMode : public NetPayload_T
    {
        MotionMode mode;

        MotionActiveMode(MotionMode mode) : mode(mode) {}

        NetPayloadType type() const
        {
            return NetPayloadType::MotionActiveMode;
        }
    };

    struct MotionUpdateXForm : public NetPayload_T
    {
        MotionXForm xform;

        MotionUpdateXForm(const MotionXForm &xform) : xform(xform) {}
        MotionUpdateXForm(MotionXForm &&xform) : xform(std::move(xform)) {}

        NetPayloadType type() const
        {
            return NetPayloadType::MotionUpdateXForm;
        }
    };
}