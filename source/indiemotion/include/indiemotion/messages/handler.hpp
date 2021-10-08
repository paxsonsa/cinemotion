// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* handler.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/session/session.hpp>
#include <indiemotion/messages/base.hpp>
#include <indiemotion/responses/base.hpp>

namespace indiemotion::messages::handling
{
    class Handler {
        public:
            virtual ~Handler() {}
            virtual std::optional<std::unique_ptr<responses::base::Response>> 
            handleMessage(std::weak_ptr<session::Session> session, 
                          std::unique_ptr<base::Message> message) = 0;
    };
}