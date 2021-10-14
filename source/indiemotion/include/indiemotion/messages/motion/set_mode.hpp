// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* motion_modes.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/messages/base.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/responses/responses.hpp>
#include <indiemotion/session/motion_mode.hpp>

namespace indiemotion::messages::motion::set_mode
{
    class Message : public base::Message
    {
    public:
        indiemotion::motion::ModeValue newMode;

        Message(indiemotion::motion::ModeValue newMode) : newMode(newMode) {}

        /**
         * @brief Returns the initsession kind
         * 
         * @return kind 
         */
        Kind kind() override
        {
            return Kind::MotionSetMode;
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
        inline static const std::string_view kind = KindNames::MotionSetMode;

        Handler() = default;
        static std::shared_ptr<Handler> make()
        {
            return std::make_shared<Handler>();
        }

        std::optional<std::unique_ptr<responses::base::Response>> handleMessage(std::weak_ptr<session::Session> sessionPtr,
                                                                                std::unique_ptr<base::Message> messagePtr) override
        {
            auto message = static_unique_pointer_cast<messages::motion::set_mode::Message>(std::move(messagePtr));
            if (auto session = sessionPtr.lock())
            {
                session->updateMotionMode(message->newMode);
                auto acknowledgeMsg = std::make_unique<responses::acknowledge::Response>(message->id());
                return acknowledgeMsg;
            }
            return {};
        }
    };
}