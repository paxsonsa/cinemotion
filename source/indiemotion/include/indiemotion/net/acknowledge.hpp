#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message/header.hpp>
#include <indiemotion/net/message/payload.hpp>

namespace indiemotion::net
{
    struct Acknowledge : public Payload_T
    {
        Acknowledge() {}

        PayloadType type() const
        {
            return PayloadType::Acknowledge;
        }
    };

    using AcknowledgeCallback = std::function<void()>;
    using AcknowledgeTimeout = std::function<void()>;

    class AcknowledgeCoordinator
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

        std::map<Identifier, record> _m_message_table{};

    public:
        AcknowledgeCoordinator() {}

        /**
         * @brief Acknowledge a message and remove it from the curator
         *
         * @param uid The unique identifier for the message to track
         */
        void acknowledge(Identifier uid)
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
                    spdlog::warn("no ack callback for message id='{}', skipping", uid);
                }
            }
            else
            {
                spdlog::error("failed to ack message id='{}': id not in curator table", uid);
            }
        }

        void queue(Identifier uid, std::function<void()> callback)
        {
            spdlog::info("queued '{}' for acknowledgement", uid);
            _m_message_table[uid] = record{callback};
        }
    };
} // namespace indiemotion::net