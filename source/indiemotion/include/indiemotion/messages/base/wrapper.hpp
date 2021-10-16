// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* wrapper.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/messages/base/payload.hpp>
#include <indiemotion/messages/kind.hpp>
#include <indiemotion/transport/wrapper.hpp>

namespace indiemotion::messages::base
{
    using Wrapper = transport::Wrapper<Payload, Kind>;

    std::unique_ptr<Wrapper> createMessage(std::string inResponseToId,
                                           std::unique_ptr<Payload> payloadPtr)
    {
        auto id = transport::generateNewId();
        auto headerPtr = std::make_unique<transport::Header>(id, inResponseToId);
        auto containerPtr = std::make_unique<Wrapper>(std::move(headerPtr), std::move(payloadPtr));

        return std::move(containerPtr);
    }

    std::unique_ptr<Wrapper> createMessage(std::unique_ptr<Payload> payloadPtr)
    {
        auto id = transport::generateNewId();
        auto headerPtr = std::make_unique<transport::Header>(id);
        auto containerPtr = std::make_unique<Wrapper>(std::move(headerPtr), std::move(payloadPtr));

        return std::move(containerPtr);
    }
}
