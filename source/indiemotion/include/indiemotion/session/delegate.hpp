// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* delegate.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/session/features.hpp>
#include <indiemotion/session/motion_mode.hpp>

namespace indiemotion::session
{
    class SessionDelegate
    {
    public:
        /**
             * @brief called when the session is just about to initialize.
             * 
             */
        virtual void sessionWillInitialize() {}

        /**
             * @brief called when the session finished initializing and is active.
             * 
             */
        virtual void sessionDidInitialize() {}

        /**
             * @brief Returns the name of the session
             * 
             * @return std::optional<std::string> 
             */
        virtual std::optional<std::string> name()
        {
            return std::nullopt;
        }

        /**
             * @brief Returns the supported features of this server
             * 
             * @return std::optional<FeatureSet> 
             */
        virtual std::optional<FeatureSet> supportedFeatures()
        {
            return std::nullopt;
        }

        /**
         * @brief Return the current list of camera names
         * 
         * This is called by the session when it needs to get a list of 
         * the available cameras in the scene.
         * 
         * @return std::vector<std::string> 
         */
        virtual std::vector<std::string> cameras()
        {
            return std::vector<std::string>();
        }

        /**
         * @brief Called whenever the motion mode is updated.
         * 
         * @param newMode 
         */
        virtual void motionModeDidUpdate(motion::ModeValue newMode) {}

        /**
         * @brief Called whenever the motion xform is update by the client.
         * 
         * use this to respond to changes in the motion of the client.
         * 
         * @param xform 
         */
        virtual void motionDidUpdate(motion::MotionXFormView xform) {}
    };
}
