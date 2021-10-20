// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* handler.hpp */
#pragma once
#include <indiemotion/messages/base/handler.hpp>
#include <indiemotion/messages/motion/set_mode/payload.hpp>
#include <indiemotion/responses.hpp>

namespace indiemotion::messages::motion::getmode
{
    class Handler : public base::Handler
    {
    public:
        std::optional<std::unique_ptr<responses::base::Response>>
        handleMessage(std::weak_ptr<session::Session> sessionPtr,
                      std::unique_ptr<base::Message> messagePtr)
        {
            if (auto session = sessionPtr.lock())
            {
                auto mode = session->motionMode();
                auto payloadPtr = std::make_unique<indiemotion::responses::motion::curmode::Payload>(mode);
                auto respPtr = responses::base::createResponse(messagePtr->header()->id(),
                                                               std::move(payloadPtr));
                return respPtr;
            }

            // TODO Error
            return {};
        }
    };
}
