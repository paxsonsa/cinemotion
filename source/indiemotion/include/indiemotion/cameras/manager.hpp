#pragma once
#include <indiemotion/cameras/camera.hpp>
#include <indiemotion/common.hpp>

namespace indiemotion {
    class CameraManager {
    private:
        std::optional<Camera> _m_activeCamera;

    public:
        CameraManager() {}

        /**
         * Get the active camera currently set.
         * @return the camera info for the active camera
         */
        std::optional<Camera> get_active_camera() const { return _m_activeCamera; }

        /**
         * Update the currently active camera
         * @param c the camera info to set as the active camera
         */
        void set_active_cameras(Camera c) { _m_activeCamera = c; }
    };
} // namespace indiemotion::cameras
