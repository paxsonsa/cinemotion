// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* factory.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/messages/acknowledge/payload.hpp>
#include <indiemotion/messages/base/message.hpp>
#include <indiemotion/protobuf.hpp>
#include <indiemotion/transport/header.hpp>

namespace indiemotion::messages
{
    class Factory
    {
    public:
        FactoryFactory() = default;

        static std::unique_ptr<base::Message> create(const protobuf::messages::ClientMessage clientMessage)
        {
            auto rawHeaderPtr = clientMessage.header();
            std::unique_ptr<transport::Header> headerPtr;
            if (rawHeaderPtr.has_responseid())
                headerPtr = std::make_unique<transport::Header>(rawHeaderPtr.id(), rawHeaderPtr.responseid());
            else
                headerPtr = std::make_unique<transport::Header>(rawHeaderPtr.id());

            std::unique_ptr<base::Payload> payloadPtr;
            switch (clientMessage.payload_case())
            {
            case protobuf::messages::ClientMessage::PayloadCase::kAcknowledge:
                payloadPtr = acknowledge::Payload::create(clientMessage.acknowledge());
                break;
            }

            return std::make_unique<base::Message>(std::move(headerPtr), std::move(payloadPtr));
        }
    };
}
