// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* motion_modes.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/messages/base.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/motion/xform.hpp>
#include <indiemotion/responses/responses.hpp>

namespace indiemotion::messages::motion::xform
{
    class Message : public base::Message
    {
    private:
        std::unique_ptr<indiemotion::motion::MotionXForm> _m_xform;

    public:
        Message(std::unique_ptr<indiemotion::motion::MotionXForm> xform) : _m_xform(std::move(xform))
        {
        }

        static std::unique_ptr<base::Message> create(std::unique_ptr<indiemotion::motion::MotionXForm> xform)
        {
            return std::make_unique<Message>(std::move(xform));
        }

        /**
         * @brief Returns the initsession kind
         * 
         * @return kind 
         */
        Kind kind() override
        {
            return Kind::MotionXForm;
        }

        bool needsAcknowledgment() override
        {
            // Require a ack when the message is recieved.
            return false;
        }

        std::unique_ptr<indiemotion::motion::MotionXForm> xform()
        {
            return std::move(_m_xform);
        }
    };

    class Handler : public handling::Handler
    {
    public:
        inline static const std::string_view kind = KindNames::MotionXForm;

        Handler() = default;
        static std::shared_ptr<Handler> make()
        {
            return std::make_shared<Handler>();
        }

        std::optional<std::unique_ptr<responses::base::Response>> handleMessage(std::weak_ptr<session::Session> sessionPtr,
                                                                                std::unique_ptr<base::Message> messagePtr) override
        {

            auto message = static_unique_pointer_cast<Message>(std::move(messagePtr));
            if (auto session = sessionPtr.lock())
            {
                session->update(std::move(message->xform()));
            }
            return {};
        }
    };
}