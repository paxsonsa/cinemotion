// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* delegate.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/session/features.hpp>

namespace indiemotion::session
{
    class SessionDelegate {
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
            virtual std::optional<std::string> name() {
                return std::nullopt;
            }

            /**
             * @brief Returns the supported features of this server
             * 
             * @return std::optional<FeatureSet> 
             */
            virtual std::optional<FeatureSet> supportedFeatures() {
                return std::nullopt;
            }
    };
}
