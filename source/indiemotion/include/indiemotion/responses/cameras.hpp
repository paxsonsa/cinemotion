// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* cameras.hpp */
#pragma once
#include <indiemotion/responses/kind.hpp>
#include <indiemotion/messages/messages.hpp>
#include <indiemotion/responses/base.hpp>
#include <indiemotion/responses/kind.hpp>

namespace indiemotion::responses
{
    namespace cameraList
    {
        struct Response : public responses::base::Response
        {
            Response(messages::base::ID mid, std::vector<std::string> names) : responses::base::Response(mid)
            {
                _m_cameraNames = names;
            }

            /**
             * @brief Returns the kind
             * 
             * @return kind 
             */
            Kind kind() override
            {
                return responses::Kind::CameraList;
            }

            bool needsAcknowledgment() override
            {
                return false;
            }

            std::vector<std::string> cameraNames()
            {
                return _m_cameraNames;
            }

        private:
            std::vector<std::string> _m_cameraNames;
        };
    }
}