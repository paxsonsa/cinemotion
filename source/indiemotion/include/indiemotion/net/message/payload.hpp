#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion::net
{
    enum class PayloadType : std::int32_t
    {
        Error = 0,
        Acknowledge,
        SessionInitilization,
        SessionShutdown,
        GetCameraList,
        CameraList,
        SetCamera,
        CameraInfo,
        SetMotionMode,

    };

    /**
     * @brief The body of a message transport, this should be subclassed
     *
     */
    class Payload_T
    {
    public:
        Payload_T() = default;
        virtual ~Payload_T() {}

        /**
     * @brief Return the kind of body
     *
     * @return Kind
     */
        virtual PayloadType type() const = 0;
    };
}