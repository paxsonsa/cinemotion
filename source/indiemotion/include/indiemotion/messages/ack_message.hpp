// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* ack_message.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/messages/message.hpp>

namespace indiemotion::messages::types
{
    struct AckMessage: public Message
    {
        messages::MessageID uid;

        AckMessage() = default;
        AckMessage(messages::MessageID uid): uid(uid) {}

        bool needsAcknowledgment() override { return false; }

        Kind getKind() override
        {
            return Kind::Ack;
        }
    };
}

namespace indiemotion::messages::handler
{
    class AckMessageHandler: public MessageHandler
    {
        public:
            AckMessageHandler() = default;

            std::optional<std::unique_ptr<Message>> handleMessage(std::weak_ptr<session::Session> session, 
                                                                  std::unique_ptr<Message> message) override
            {
                return {};
            }
    };
}