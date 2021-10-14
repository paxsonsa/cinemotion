// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* error.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/messages/messages.hpp>
#include <indiemotion/responses/base.hpp>

namespace indiemotion::messages::errors
{
    class ErrorResponse : public responses::base::Response
    {
    private:
        std::string _m_eid;
        std::string _m_message;

    public:
        ErrorResponse(response::base::ID messageId, std::string errorId, std::string message) : responses::base::Response(messageId)
        {
            _m_eid = errorId;
            _m_message = message;
        }
        ErrorResponse(std::string errorId, std::string message) : responses::base::Response()
        {
            _m_eid = errorId;
            _m_message = message;
        }

        /**
         * @brief Returns the initsession kind
         * 
         * @return kind 
         */
        responses::Kind kind() override
        {
            return responses::Kind::Error;
        }

        /**
         * @brief Does this message require acknowledgment messagges
         * 
         * @return true 
         * @return false 
         */
        bool needsAcknowledgment() override { return false; };

        /**
         * @brief Returns the error type
         * 
         * @return std::string 
         */
        std::string type() const { return _m_eid; }

        /**
         * @brief returns the fully qualified error message.
         * 
         * @return std::string 
         */
        std::string error() const
        {
            std::string error = _m_eid;
            error += ": ";
            error += _m_message;
            return error;
        }
    };

}
