// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* container.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/responses/base/payload.hpp>
#include <indiemotion/responses/kind.hpp>
#include <indiemotion/transport/container.hpp>
#include <indiemotion/transport/header.hpp>

#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

namespace indiemotion::responses::base
{
    using Container = transport::Container<Payload, Kind>;

    std::unique_ptr<Container> createContainer(std::string inResponseToId,
                                               std::unique_ptr<Payload> payloadPtr)
    {
        boost::uuids::random_generator generator;
        boost::uuids::uuid uuid = generator();
        auto mid = boost::uuids::to_string(uuid);

        auto headerPtr = std::make_unique<transport::Header>(mid, inResponseToId);
        auto containerPtr = std::make_unique<Container>(std::move(headerPtr), std::move(payloadPtr));

        return std::move(containerPtr);
    }
}