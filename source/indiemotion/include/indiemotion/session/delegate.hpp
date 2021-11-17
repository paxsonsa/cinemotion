#pragma once
#include <indiemotion/motion/xform.hpp>
#include <indiemotion/cameras/camera.hpp>
#include <indiemotion/session/controller.hpp>

namespace indiemotion
{
    class SessionControllerDelegate
    {
    public:
        virtual ~SessionControllerDelegate() {}

        // ---------------------------------------------------------------------
        // Camera Operations

        /**
         * @brief Return the current list of camera names
         *
         * This is called by the session when it needs to get a list of
         * the available cameras in the scene.
         *
         * @return std::vector<std::string>
         */
        virtual std::vector<Camera> get_available_cameras()
        {
            return std::vector<Camera>();
        }

        /**
         * @brief Get the Camera By ID
         * 
         * If the camera with that ID does not exist, then an empty optional
         * is returned.
         * 
         * @param id The camera id to get
         * @return std::optional<cameras::Camera> 
         */
        virtual std::optional<Camera> get_camera_by_name(std::string name)
        {
            return {};
        }

        /**
         * @brief Called when the active active camera is set
         * 
         * @param camera 
         */
        virtual void did_set_active_camera(Camera camera) {}

        /**
         * @brief Called when the motion mode is updated
         * 
         * @param m 
         */
        virtual void did_set_motion_mode(MotionMode m) {}

        /**
         * Invoked when a new motion update is received.
         * @param m
         */
        virtual void did_receive_motion_update(MotionXForm m) {}

        /**
         * Invoke at the beginning of the session shutdown process.
         */
        virtual void will_shutdown_session() {}

        /**
         * Called right before the session is initialized.
         *
         * The session state is undefined at this point, do not access it.
         */
        virtual void will_start_session() {}

        /**
         * Called after the session is successfully started.
         */
        virtual void did_start_session() {}
    };
}