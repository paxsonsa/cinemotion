// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* response.hpp 
*/
#pragma once
#include <chrono>

#include <indiemotion/_common.hpp>

namespace indiemotion::messages
{
    namespace message
    {
        /**
         * @brief helper type for message ids
         */
        using ID = uint32_t;

        /**
         * @brief Represents the kind of messages available
         */
        enum class kind
        {
            Acknowledgment = 0,
            // Error = 1,

            ListCameras = 200
        };

        /**
         * @brief return a string name for the given kind
         * 
         * @param k the kind to transform into string
         * @return std::string 
         */
        std::string kindToStr(kind k)
        {
            switch (k)
            {
            case kind::Acknowledgment:
                return "Acknowledgment";
            // case kind::Error:
            //     return "Error";
            case kind::ListCameras:
                return "ListCameras";
            }
        }

        /**
         * @brief Represents a message that is recieved from the client
         * 
         */
        class Message
        {
        private:
            ID _m_id;
            std::optional<message::ID> _m_messageId;

        public:
            Message()
            {
                using namespace std::chrono;
                _m_id = duration_cast<milliseconds>(
                             system_clock::now().time_since_epoch())
                             .count();
            }

            Message(message::ID mid): Message()
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
            std::optional<message::ID> messageId()
            {
                return _m_messageId;
            }

            /**
             * @brief Get the kind object
             * 
             * @return kind 
             */
            virtual kind kind() = 0;

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
    }

    namespace response
    {
        /**
         * @brief helper type for message ids
         */
        using ID = uint32_t;

        /**
         * @brief Represents the kind of messages available
         */
        enum class kind
        {
            Acknowledgment = 0,
            Error = 1,

            InitSession = 100,

            ListCameras = 200
        };

        /**
         * @brief return a string name for the given kind
         * 
         * @param k the kind to transform into string
         * @return std::string 
         */
        std::string kindToStr(kind k)
        {
            switch (k)
            {
            case kind::Acknowledgment:
                return "Acknowledgment";
            case kind::Error:
                return "Error";
            case kind::InitSession:
                return "InitSession";
            case kind::ListCameras:
                return "ListCameras";
            }
        }

        /**
         * @brief Represents a message that is recieved from the client
         * 
         */
        class Response
        {
        private:
            ID _m_id;
            std::optional<message::ID> _m_messageId;

        public:
            Response()
            {
                using namespace std::chrono;
                _m_id = duration_cast<milliseconds>(
                             system_clock::now().time_since_epoch())
                             .count();
            }

            Response(message::ID mid): Response()
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
            std::optional<message::ID> messageId()
            {
                return _m_messageId;
            }

            /**
             * @brief Get the kind object
             * 
             * @return kind 
             */
            virtual kind kind() = 0;

            /**
             * @brief Does this message require acknowledgment messagges
             * 
             * @return true 
             * @return false 
             */
            virtual bool needsAcknowledgment() = 0;
        };
    }
} // namespace indiemotion::messages::response
