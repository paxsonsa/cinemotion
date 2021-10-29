#pragma once
#include <indiemotion/cameras/camera.hpp>
#include <indiemotion/session/session.hpp>

namespace indiemotion::session
{
    class Delegate
    {
    public:
        virtual ~Delegate() {}

        /**
         * @brief Return the current list of camera names
         *
         * This is called by the session when it needs to get a list of
         * the available cameras in the scene.
         *
         * @return std::vector<std::string>
         */
        virtual std::vector<cameras::Camera> cameras()
        {
            return std::vector<cameras::Camera>();
        }
    };
}