// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* payload.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/messages/base/payload.hpp>
#include <indiemotion/messages/kind.hpp>
#include <indiemotion/protobuf.hpp>

namespace indiemotion::messages::cameras::list
{
    class Payload : public base::Payload
    {
    public:
        static std::unique_ptr<Payload> create([[maybe_unused]] const protobuf::messages::ListCameras rawPayload)
        {

            return std::make_unique<Payload>();
        }

        Payload() = default;

        Kind kind() override
        {
            return Kind::ListCameras;
        }
    };
}