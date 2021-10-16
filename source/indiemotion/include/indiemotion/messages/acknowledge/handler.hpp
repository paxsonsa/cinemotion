// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* handler.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/messages/acknowledge/payload.hpp>
#include <indiemotion/messages/handlers/handler.hpp>
#include <indiemotion/responses/base/wrapper.hpp>
#include <indiemotion/session/session.hpp>

namespace indiemotion::messages::acknowledge
{
    class Handler : base::Handler
    {
    public:
        std::optional<std::unique_ptr<responses::base::Response>>
        handleMessage(std::weak_ptr<session::Session> session,
                      std::unique_ptr<base::Wrapper> message)
        {
            return {};
        }
    };
}
