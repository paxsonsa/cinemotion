// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* payload.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/messages/base/payload.hpp>
#include <indiemotion/messages/kind.hpp>
#include <indiemotion/protobuf.hpp>

namespace indiemotion::messages::acknowledge
{
    class Payload : public base::Payload
    {
    private:
        const std::string _m_message;
        const bool _m_ok;

    public:
        static std::unique_ptr<Payload> create(const protobuf::messages::Acknowledge rawPayload)
        {
            auto ok = rawPayload.ok();
            std::string message = "";
            if (rawPayload.has_message())
                message = rawPayload.message();

            return std::make_unique<Payload>(ok, message);
        }

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
