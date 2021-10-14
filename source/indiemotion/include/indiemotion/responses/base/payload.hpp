// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* payload.hpp */
#pragma once
#include <indiemotion/responses/kind.hpp>
#include <indiemotion/transport/payload.hpp>

namespace indiemotion::responses::base
{
    using Payload = transport::Payload<Kind>;
}