#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>

namespace indiemotion::net
{
    enum class ErrorType : std::uint32_t
    {
        UnexpectedError = 1,
        InvalidMessage,
    };

    struct Error : public Payload_T
    {
        ErrorType errorType;
        std::string message;

        Error(ErrorType errorType, std::string msg) : errorType(errorType), message(msg) {}

        PayloadType type() const
        {
            return PayloadType::Error;
        }
    };
} // namespace indiemotion::net