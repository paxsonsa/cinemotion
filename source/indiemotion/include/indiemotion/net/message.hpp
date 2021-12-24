// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
#pragma once
#include <indiemotion/errors.hpp>
#include <indiemotion-protobufs/messages.pb.h>
#include <indiemotion-protobufs/payload.v1.pb.h>

#include <boost/uuid/random_generator.hpp>
#include <boost/uuid/uuid_io.hpp>

namespace indiemotion
{
    using NetMessage = indiemotion::protobuf::Message;
    namespace netPayloadsV1 = indiemotion::protobuf::payloads::v1;

    /**
     * @brief Generate is new NetIdentifier
     *
     * @return std::string
     */
    std::string net_generate_new_identifier_string() {
        boost::uuids::random_generator generator;
        boost::uuids::uuid uuid = generator();
        return boost::uuids::to_string(uuid);
    }

    /**
     * Make a new message and generate the ID.
     * @return unique pointer to a new message
     */
    NetMessage net_make_message() {
        auto id = net_generate_new_identifier_string();
        NetMessage m;
        m.mutable_header()->set_id(id);
        return std::move(m);
    }

    /**
     * Make a new message and generate the ID.
     * @return unique pointer to a new message
     */
    NetMessage net_make_message_with_response_id(std::string responseId) {
        auto id = net_generate_new_identifier_string();
        NetMessage m;
        m.mutable_header()->set_id(id);
        m.mutable_header()->set_responseid(responseId);
        return std::move(m);
    }

    /**
     * Create an Error Response Message
     * @param messageID The message ID that causes the exception
     * @param exception The exception to generate an error message from
     * @return An error message that is in response to some message id
     */
    NetMessage net_make_error_response_from_exception(const std::string messageID, const Exception &exception)
    {
        auto message = net_make_message_with_response_id(messageID);
        auto error = message.mutable_error();
        error->set_type(exception.type);
        error->set_message(exception.message);
        error->set_is_fatal(exception.is_fatal);

        return std::move(message);
    }

    std::string net_get_message_payload_name(const NetMessage &message)
    {
        auto desc = message.descriptor();
        auto field = desc->FindFieldByNumber(message.payload_case());
        return field->full_name();
    }


}
