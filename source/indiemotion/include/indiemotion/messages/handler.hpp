// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* handler.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/session/session.hpp>
#include <indiemotion/messages/message.hpp>

namespace indiemotion::messages
{
    class Handler {
        public:
            virtual ~Handler() {}
            virtual std::optional<std::unique_ptr<messages::response::Response>> 
            handleMessage(std::weak_ptr<session::Session> session, 
                          std::unique_ptr<messages::message::Message> message) = 0;
    };
}