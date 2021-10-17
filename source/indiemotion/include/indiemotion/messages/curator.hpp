// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* manager.hpp

*/
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/messages/base/message.hpp>

namespace indiemotion::messages
{
    /**
     * @brief A class for managing in-flight messages awaiting acknowledgment.
     *
     */
    class Curator
    {
    private:
        /**
         * @brief A small record for storing messages
         *
         */
        struct record
        {
            /**
             * @brief The optionalfunction that should be invoked when the message is acknowledged.
             *
             */
            std::optional<std::function<void()>> callback;
        };

        std::map<std::string, record> _m_message_table{};

    public:
        /**
         * @brief Acknowledge a message and remove it from the curator
         *
         * @param uid The unique identifier for the message to track
         */
        void acknowledge(std::string uid)
        {
            if (_m_message_table.count(uid) > 0)
            {
                auto record = _m_message_table[uid];
                if (auto callback = record.callback)
                {
                    callback.value()();
                }
                else
                {
                    spdlog::warn("no ack callback for message id='{}': id not in curator table", uid);
                }
            }
            else
            {
                spdlog::error("failed to ack message id='{}': id not in curator table", uid);
            }
        }

        void queue(std::string uid, std::function<void()> callback)
        {
            spdlog::info("queued '{}' for acknowledgement", uid);
            _m_message_table[uid] = record{
                callback};
        }
    };

}
