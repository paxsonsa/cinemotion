// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* wrapper.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/responses/base/payload.hpp>
#include <indiemotion/responses/kind.hpp>
#include <indiemotion/transport/header.hpp>
#include <indiemotion/transport/wrapper.hpp>

namespace indiemotion::responses::base
{
    using Wrapper = transport::Wrapper<Payload, Kind>;

    std::unique_ptr<Wrapper> createContainer(std::string inResponseToId,
                                             std::unique_ptr<Payload> payloadPtr)
    {
        auto mid = transport::generateNewId();
        auto headerPtr = std::make_unique<transport::Header>(mid, inResponseToId);
        auto containerPtr = std::make_unique<Wrapper>(std::move(headerPtr), std::move(payloadPtr));

        return std::move(containerPtr);
    }
}
