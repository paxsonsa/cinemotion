#pragma once

#include <indiemotion/cameras/camera.hpp>
#include <indiemotion/common.hpp>

namespace indiemotion {
    class CameraManager {
    private:
        std::optional<Camera> _m_activeCamera;

    public:
        CameraManager() {}

        std::optional<Camera> get_active_cameras() const { return _m_activeCamera; }
        void set_active_cameras(Camera c) { _m_activeCamera = c; }
    };
} // namespace indiemotion::cameras
