// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* cameras.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/messages/message.hpp>

namespace indiemotion::messages::cameras
{

    /**
     * @brief Message from client to list the cameras
     * 
     */
    struct ListCamerasMessage : public message::Message
    {

        ListCamerasMessage() = default;

        /**
         * @brief Returns the kind
         * 
         * @return kind 
         */
        message::kind kind() override
        {
            return message::kind::ListCameras;
        }
    };

    struct CameraListResponse : public response::Response
    {
        CameraListResponse(message::ID mid, std::vector<std::string> names) : response::Response(mid)
        {
            _m_cameraNames = names;
        }

        /**
         * @brief Returns the kind
         * 
         * @return kind 
         */
        response::kind kind() override {
            return response::kind::ListCameras;
        }

        bool needsAcknowledgment() override
        {
            return false;
        }

        std::vector<std::string> cameraNames()
        {
            return _m_cameraNames;
        }

    private:
        std::vector<std::string> _m_cameraNames;
    };

    class ListCamerasMessageHandler : public Handler
    {
    public:
        static std::shared_ptr<Handler> make()
        {
            return std::make_shared<ListCamerasMessageHandler>();
        }

        ListCamerasMessageHandler() = default;

        static constexpr std::string_view kind = "ListCameras";

        std::optional<std::unique_ptr<messages::response::Response>> handleMessage(std::weak_ptr<session::Session> session_ptr,
                                                                                   std::unique_ptr<message::Message> message) override
        {
            spdlog::info("hello, list cameras");
            if (auto session = session_ptr.lock()){
                auto cameras = session->cameras();
                auto p_msg = std::make_unique<CameraListResponse>(message->messageId().value(), cameras);
                return p_msg;
            }
            return {};
        }
    };

} // namespace indiemotion::messages::cameras
