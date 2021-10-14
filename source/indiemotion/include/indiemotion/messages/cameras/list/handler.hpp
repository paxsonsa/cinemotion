// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* handler.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/messages/base/handler.hpp>
#include <indiemotion/responses/responses.hpp>
#include <indiemotion/session/session.hpp>

namespace indiemotion::messages::cameras::list
{
    class Handler : public base::Handler
    {
    public:
        std::optional<std::unique_ptr<responses::base::Wrapper>>
        handleMessage(std::weak_ptr<session::Session> sessionPtr,
                      std::unique_ptr<base::Wrapper> messagePtr)
        {
            if (auto session = sessionPtr.lock())
            {
                auto cameras = session->cameras();
                auto payloadPtr = std::make_unique<responses::cameras::list::Payload>(cameras);
                auto ctnPtr = responses::base::createContainer(messagePtr->header().lock()->id(), std::move(payloadPtr));
                return ctnPtr;
            }
            return {};
        }
    };
}
