// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* errors.hpp */
#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion
{
    enum class ErrorType : std::uint32_t
    {
        UnknownError = 0,
        InvalidMessage,
    };

    std::string ErrorType_toString(ErrorType e)
    {
        switch (e)
        {
        case ErrorType::UnknownError:
            return "UnknownError";
        case ErrorType::InvalidMessage:
            return "InvalidMessage";
        }
    }

    class SessionError : public std::exception
    {
    public:
        ErrorType etype;
        std::string message;

    private:
        std::string _m_error;

    public:
        SessionError(ErrorType type, std::string message) noexcept : etype(type), message(message)
        {
            _m_error = ErrorType_toString(etype);
            _m_error += ": " + message;
        }

        const char *what() const noexcept
        {
            return _m_error.c_str();
        }
    };

} // namespace indiemotion::errors
