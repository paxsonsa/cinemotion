// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* acknowledge.hpp 
*/
#pragma once
#include <indiemotion/messages/message.hpp>
#include <indiemotion/messages/handler.hpp>

namespace indiemotion::messages::acknowledge
{
    class AcknowledgeMessage : public message::Message
    {
    public:
        AcknowledgeMessage(message::ID messageId) : message::Message(messageId) {}

        /**
         * @brief Returns the initsession kind
         * 
         * @return kind 
         */
        message::kind kind() override
        {
            return message::kind::Acknowledgment;
        }

        bool needsAcknowledgment() override
        {
            // Require a ack when the message is recieved.
            return false;
        }
    };

    class AcknowledgeResponse : public response::Response
    {
    public:
        AcknowledgeResponse(response::ID messageId) : response::Response(messageId) {}

        /**
         * @brief Returns the initsession kind
         * 
         * @return kind 
         */
        response::kind kind() override
        {
            return response::kind::Acknowledgment;
        }

        bool needsAcknowledgment() override
        {
            // Require a ack when the message is recieved.
            return false;
        }
    };

    class AckMessageHandler : public Handler
    {
    public:
        AckMessageHandler() = default;

        static constexpr std::string_view kind = "Acknowledge";

        std::optional<std::unique_ptr<messages::response::Response>> handleMessage(std::weak_ptr<session::Session> session,
                                                                                   std::unique_ptr<message::Message> message) override
        {
            return {};
        }
    };

    // class AcknowledgeMessageHandler: public messages::handler::Handler {};
}
