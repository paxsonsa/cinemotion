// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* payload.hpp */

#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/responses/base/payload.hpp>
#include <indiemotion/responses/kind.hpp>

namespace indiemotion::responses::cameras::list
{
    class Payload : public base::Payload
    {
    private:
        std::vector<std::string> _m_cameraNames;

    public:
        Payload(std::vector<std::string> names) : _m_cameraNames(names)
        {
        }

        Kind kind() override
        {
            return Kind::CameraList;
        }

        std::vector<std::string> cameraNames()
        {
            return _m_cameraNames;
        }
    };
}