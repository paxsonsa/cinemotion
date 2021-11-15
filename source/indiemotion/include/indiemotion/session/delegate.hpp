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
        virtual std::vector<Camera> getAvailableCameras()
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
        virtual std::optional<Camera> getCameraById(std::string id)
        {
            return {};
        }

        /**
         * @brief Called when the active active camera is set
         * 
         * @param camera 
         */
        // TODO Return an optional error.
        virtual void didSetActiveCamera(Camera camera) {}

        // ----------------------------------------------------------------
        // Motion Mode Operations

        /**
         * @brief Called when the motion mode is updated
         * 
         * @param m 
         */
        virtual void didMotionSetMode(MotionMode m) {}

        // ----------------------------------------------------------------
        // Motion XForm Operations
        virtual void receivedMotionUpdate(MotionXForm m) {}

        // ----------------------------------------------------------------
        // Session Operations
        virtual void sessionWillShutdown() {}
        virtual void sessionWillStart() {}
        virtual void sessionDidStart() {}
    };
}