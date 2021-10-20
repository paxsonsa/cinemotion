// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* payload.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/responses/base/payload.hpp>
#include <indiemotion/responses/kind.hpp>

namespace indiemotion::responses::acknowledge
{
    class Payload : public base::Payload
    {
    private:
        const std::string _m_message;
        const bool _m_ok;

    public:
        static std::unique_ptr<Payload> create(const bool ok, const std::string message)
        {
            return std::make_unique<Payload>(ok, message);
        }

        Payload(const bool ok, const std::string message) : _m_ok(ok), _m_message(message)
        {
        }

        bool ok()
        {
            return _m_ok;
        }

        Kind kind() override
        {
            return Kind::Acknowledgment;
        }

        std::string message()
        {
            return _m_message;
        }
    };
}
