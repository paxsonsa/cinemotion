// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* manager.hpp 

*/
#include <indiemotion/_common.hpp>
#include <indiemotion/messages/message.hpp>

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

        std::map<UID, record> _m_message_table;

    public:
        /**
             * @brief Acknowledge a message and remove it from the curator
             * 
             * @param uid The unique identifier for the message to track
             */
        void acknowledge(UID uid)
        {   
            auto record = _m_message_table[uid];
            if (auto callback = record.callback)
                callback.value()();
        }

        void queue(UID uid, std::function<void()> callback)
        {
            _m_message_table[uid] = record{
                callback
            };
        }
    };

}