#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion::net
{
    enum class PayloadType : std::int32_t
    {
        // ---------------------------------------------------------
        // General Payload Types
        Error = 0,
        Acknowledge,

        // ---------------------------------------------------------
        // Sesion Payload Types
        SessionInitilization,
        SessionShutdown,

        // ---------------------------------------------------------
        // Camera Payload Types
        GetCameraList,
        CameraList,
        SetCamera,
        CameraInfo,

        // ---------------------------------------------------------
        // Motion Payload Types
        SetMotionMode,
        UpdateMotionXForm,

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