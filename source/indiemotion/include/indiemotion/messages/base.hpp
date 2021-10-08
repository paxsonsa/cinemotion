// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* response.hpp 
*/
#pragma once
#include <chrono>

#include <indiemotion/_common.hpp>
#include <indiemotion/messages/kind.hpp>

namespace indiemotion::messages::base
{
    /**
     * @brief helper type for message ids
     */
    using ID = uint32_t;

    /**
     * @brief Represents a message that is recieved from the client
     * 
     */
    class Message
    {
    private:
        ID _m_id;
        std::optional<ID> _m_messageId;

    public:
        Message()
        {
            using namespace std::chrono;
            _m_id = duration_cast<milliseconds>(
                        system_clock::now().time_since_epoch())
                        .count();
        }

        Message(ID mid) : Message()
        {
            _m_messageId = mid;
        }

        virtual ~Message() {}

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
        std::optional<ID> messageId()
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
        virtual bool needsAcknowledgment()
        {
            return false;
        };
    };
} // namespace indiemotion::messages::response
