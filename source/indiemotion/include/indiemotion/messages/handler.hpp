// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* handler.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/session/session.hpp>
#include <indiemotion/messages/message.hpp>

namespace indiemotion::messages::handler
{
    class MessageHandler {
        public:
            virtual std::optional<std::unique_ptr<Message>> handleMessage(std::weak_ptr<session::Session> session, 
                                                                          std::unique_ptr<Message> message) = 0;
    };
}