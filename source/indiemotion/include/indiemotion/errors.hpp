// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* errors.hpp */
#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion::errors
{
    class SessionError : public std::exception
    {
    public:
        std::string etype;
        std::string message;

    private:
        std::string _m_error;

    public:
        SessionError(std::string type, std::string message) noexcept : etype(type), message(message)
        {
            _m_error = etype + ": " + message;
        }

        const char *what() const noexcept
        {
            return _m_error.c_str();
        }
    };

} // namespace indiemotion::errors
