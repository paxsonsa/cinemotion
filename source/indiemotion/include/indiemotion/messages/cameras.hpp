// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* cameras.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/messages/kind.hpp>
#include <indiemotion/messages/base.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/responses/base.hpp>
#include <indiemotion/responses/cameras.hpp>


namespace indiemotion::messages::listCameras
{

    /**
     * @brief Message from client to list the cameras
     * 
     */
    struct Message : public base::Message
    {

        Message() = default;

        /**
         * @brief Returns the kind
         * 
         * @return kind 
         */
        Kind kind() override
        {
            return Kind::ListCameras;
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

        inline static const std::string_view kind = KindNames::ListCameras;

        std::optional<std::unique_ptr<responses::base::Response>> handleMessage(std::weak_ptr<session::Session> session_ptr,
                                                                                    std::unique_ptr<base::Message> message) override
        {
            if (auto session = session_ptr.lock()){
                auto cameras = session->cameras();
                auto p_msg = std::make_unique<responses::cameraList::Response>(message->id(), cameras);
                return p_msg;
            }
            return {};
        }
    };

} // namespace indiemotion::messages::listCameras
