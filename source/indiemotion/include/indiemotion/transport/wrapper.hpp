// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* wrapper.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/transport/header.hpp>
#include <indiemotion/transport/payload.hpp>

namespace indiemotion::transport
{
    template <typename Payload_T, typename Kind_T,
              typename = std::enable_if_t<std::is_base_of_v<Payload<Kind_T>, Payload_T>>>
    class Wrapper
    {
    public:
    private:
        std::shared_ptr<Header> _m_header;
        std::shared_ptr<Payload_T> _m_payload;

    public:
        Wrapper(std::unique_ptr<Header> headerPtr,
                std::unique_ptr<Payload_T> payloadPtr) : _m_header(std::move(headerPtr)),
                                                         _m_payload(std::move(payloadPtr))
        {
        }

        std::weak_ptr<Header> header()
        {
            return _m_header;
        }

        std::weak_ptr<Payload_T> payload()
        {
            return _m_payload;
        }

        Kind_T payloadKind()
        {
            return _m_payload->kind();
        }
    };
}
