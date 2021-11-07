#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/net/error.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/error.hpp>
#include <indiemotion/net/translator.hpp>
#include <indiemotion/net/protobuf.hpp>

TEST_CASE("Translate Error Messages to Protobuf")
{
    auto translator = indiemotion::net::MessageTranslator();

    auto payload = std::make_unique<indiemotion::net::Error>(
        indiemotion::net::Error::Type::InvalidMessage,
        "some kind of invalid message error"
        );
    auto message = indiemotion::net::createMessage(
        indiemotion::net::Identifier("somemessageID"),
        indiemotion::net::Identifier("responseID"),
        std::move(payload));

    auto protobuf = translator.translateMessage(std::move(message));

    REQUIRE(protobuf.has_error());

    auto error = protobuf.error();
    REQUIRE(error.type() == indiemotion::net::Error::Type::InvalidMessage);
    REQUIRE(error.message() == "some kind of invalid message error");

    REQUIRE(protobuf.header().id() == "somemessageID");
    REQUIRE(protobuf.header().has_responseid());
    REQUIRE(protobuf.header().responseid() == "responseID");
}