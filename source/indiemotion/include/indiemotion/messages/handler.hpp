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
    class MessageHandler {
        virtual std::optional<Message> handle_message(std::weak_ptr<session::Session> session, Message message) = 0;
    };
}