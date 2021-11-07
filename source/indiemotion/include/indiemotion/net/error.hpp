#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>

namespace indiemotion::net
{
    struct Error : public Payload_T
    {
        struct Type
        {
            static const std::string UnexpectedError;
            static const std::string InvalidMessage;
            static const std::string CannotProcessMessage;
        };
        std::string errorType;
        std::string message;

        Error(std::string errorType, std::string msg) : errorType(errorType), message(msg) {}

        PayloadType type() const
        {
            return PayloadType::Error;
        }
    };

    const std::string Error::Type::UnexpectedError = "UnexpectedError";
    const std::string Error::Type::InvalidMessage = "InvalidMessage";
    const std::string Error::Type::CannotProcessMessage = "CannotProcessMessage";
} // namespace indiemotion::net