// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* errors.hpp */
#include <indiemotion/_common.hpp>

namespace indiemotion::errors
{
    class SessionError : public std::exception
    {
    private:
        std::string _m_type;
        std::string _m_message;

    public:
        SessionError(std::string type, std::string message) noexcept : _m_type(type), _m_message(message) {}

        const char *what() const noexcept
        {
            std::string message = _m_type + ": " + _m_message;
            return message.c_str();
        }
    };

} // namespace indiemotion::errors
