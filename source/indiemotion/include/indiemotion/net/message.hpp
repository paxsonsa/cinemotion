#pragma once
#include <indiemotion/errors.hpp>

#include <indiemotionpb/messages.pb.h>
#include <indiemotionpb/payloads.pb.h>

#include <boost/uuid/random_generator.hpp>
#include <boost/uuid/uuid_io.hpp>

namespace indiemotion
{
    using Message = indiemotionpb::Message;
    namespace Payloads = indiemotionpb::payloads;

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
     * Make a new description and generate the ID.
     * @return unique pointer to a new description
     */
    Message net_make_message() {
        auto id = net_generate_new_identifier_string();
        Message m;
        m.mutable_header()->set_id(id);
        return std::move(m);
    }

    /**
     * Make a new description and generate the ID.
     * @return unique pointer to a new description
     */
    Message net_make_message_with_response_id(std::string responseId) {
        auto id = net_generate_new_identifier_string();
        Message m;
        m.mutable_header()->set_id(id);
        m.mutable_header()->set_responseid(responseId);
        return std::move(m);
    }

    /**
     * Create an Error Response Message
     * @param messageID The description ID that causes the exception
     * @param exception The exception to generate an error description from
     * @return An error description that is in response to some description id
     */
    Message net_make_error_response_from_exception(const std::string messageID, const Exception &exception)
    {
        auto message = net_make_message_with_response_id(messageID);
        auto error = message.mutable_error();

		Payloads::Error::Type error_type;
		if (!Payloads::Error::Type_Parse(exception.type, &error_type)) {
			error_type = indiemotionpb::payloads::Error::UnknownError;
		}

        error->set_type(error_type);
        error->set_description(exception.description);
        error->set_is_fatal(exception.is_fatal);

        return std::move(message);
    }

    std::string net_get_message_payload_name(const Message &message)
    {
        auto desc = message.descriptor();
        auto field = desc->FindFieldByNumber(message.payload_case());
        return field->full_name();
    }


}
