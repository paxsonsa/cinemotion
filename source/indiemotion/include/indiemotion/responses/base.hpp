// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* responses.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/messages/kind.hpp>
#include <indiemotion/responses/kind.hpp>

namespace indiemotion::responses::base
{
    /**
     * @brief helper type for message ids
     */
    using ID = uint32_t;

    /**
     * @brief Represents a message that is recieved from the client
     * 
     */
    class Response
    {
    private:
        ID _m_id;
        std::optional<messages::base::ID> _m_messageId;

    public:
        Response()
        {
            using namespace std::chrono;
            _m_id = duration_cast<milliseconds>(
                        system_clock::now().time_since_epoch())
                        .count();
        }

        Response(messages::base::ID mid) : Response()
        {
            _m_messageId = mid;
        }

        virtual ~Response() {}

        /**
         * @brief Get the Id object
         * 
         * @return UID 
         */
        ID id()
        {
            return _m_id;
        }

        /**
         * @brief Get the messageId this is a response too
         * 
         * @return std::optional<message::ID>
         */
        std::optional<messages::base::ID> messageId()
        {
            return _m_messageId;
        }

        /**
         * @brief Get the kind object
         * 
         * @return kind 
         */
        virtual Kind kind() = 0;

        /**
         * @brief Does this message require acknowledgment messagges
         * 
         * @return true 
         * @return false 
         */
        virtual bool needsAcknowledgment() = 0;
    };

}