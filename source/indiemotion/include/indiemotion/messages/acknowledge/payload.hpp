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
        const protobuf::messages::Acknowledge _m_rawPayload;

    public:
        static std::unique_ptr<Payload> create(const protobuf::messages::Acknowledge rawPayload)
        {

            return std::make_unique<Payload>(rawPayload);
        }

        Payload(const protobuf::messages::Acknowledge rawPayload) : _m_rawPayload(rawPayload)
        {
        }

        bool ok()
        {
            return _m_rawPayload.ok();
        }

        Kind kind() override
        {
            return Kind::Acknowledgment;
        }

        std::string message()
        {
            if (_m_rawPayload.has_message())
            {
                return _m_rawPayload.message();
            }
            else
            {
                return "";
            }
        }
    };
}