// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* acknowledge.hpp 
*/
#pragma once
#include <indiemotion/messages/kind.hpp>
#include <indiemotion/messages/base.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/responses/base.hpp>

namespace indiemotion::messages::acknowledge
{
    class Message : public base::Message
    {
    public:
        Message(base::ID messageId) : base::Message(messageId) {}

        /**
         * @brief Returns the initsession kind
         * 
         * @return kind 
         */
        Kind kind() override
        {
            return Kind::Acknowledgment;
        }

        bool needsAcknowledgment() override
        {
            // Require a ack when the message is recieved.
            return false;
        }
    };

    class Handler : public handling::Handler
    {
    public:

        static std::shared_ptr<Handler> make()
        {
            return std::make_shared<Handler>();
        }

        Handler() = default;

        inline static const std::string_view kind = KindNames::Acknowledgment;

        std::optional<std::unique_ptr<responses::base::Response>> handleMessage(std::weak_ptr<session::Session> session,
                                                                                std::unique_ptr<base::Message> message) override
        {
            return {};
        }
    };

    // class AcknowledgeMessageHandler: public messages::handler::Handler {};
}
