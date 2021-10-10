// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* motion_modes.hpp */
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/messages/base.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/responses/responses.hpp>

namespace indiemotion::messages::motion::get_mode
{
    class Message : public base::Message
    {
    public:
        Message() = default;

        /**
         * @brief Returns the initsession kind
         * 
         * @return kind 
         */
        Kind kind() override
        {
            return Kind::MotionGetMode;
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
        inline static const std::string_view kind = KindNames::MotionGetMode;

        Handler() = default;
        static std::shared_ptr<Handler> make()
        {
            return std::make_shared<Handler>();
        }

        std::optional<std::unique_ptr<responses::base::Response>> handleMessage(std::weak_ptr<session::Session> sessionPtr,
                                                                                std::unique_ptr<base::Message> message) override
        {
            if (auto session = sessionPtr.lock())
            {
                auto mode = session->motionMode();
                auto msgPtr = std::make_unique<responses::motion::current_mode::Response>(message->id(), mode);
                return std::move(msgPtr);
            }
            return {};
        }
    };
}