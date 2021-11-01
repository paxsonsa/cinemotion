#pragma once

#include <indiemotion/cameras/camera.hpp>
#include <indiemotion/common.hpp>

namespace indiemotion::cameras
{
    class CameraManager
    {
    private:
        std::optional<Camera> _m_activeCamera;

    public:
        CameraManager() {}

        std::optional<Camera> getActiveCamera() const { return _m_activeCamera; }
        void setActiveCamera(Camera c) { _m_activeCamera = c; }
    };
} // namespace indiemotion::cameras
