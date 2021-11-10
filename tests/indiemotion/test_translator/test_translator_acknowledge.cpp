#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/net/acknowledge.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/translator.hpp>
#include <indiemotion/net/protobuf.hpp>

TEST_CASE("Translate Acknowledge Messages")
{
    auto translator = indiemotion::net::MessageTranslator();

    auto payload = std::make_unique<indiemotion::net::Acknowledge>();
    auto message = indiemotion::net::makeMessageWithResponseID(
        indiemotion::net::Identifier("somemessageID"),
        std::move(payload));
    auto protobuf = translator.translateMessage(std::move(message));

    SUBCASE("protobuf message has acknowledge payload")
    {
        REQUIRE(protobuf.has_acknowledge());
    }

    SUBCASE("protobuf message has responseId set")
    {
        REQUIRE(protobuf.header().has_responseid());
        REQUIRE(protobuf.header().responseid() == "somemessageID");
    }
}

TEST_CASE("Translate Acknowledge Protobuf")
{
    auto translator = indiemotion::net::MessageTranslator();

    indiemotion::protobuf::messages::Message protobuf;
    auto header = protobuf.mutable_header();
    header->set_id("someid");
    header->set_responseid("someresponseid");
    protobuf.mutable_acknowledge();

    std::shared_ptr<indiemotion::net::Message> message = translator.translateProtobuf(std::move(protobuf));

    SUBCASE("protobuf message has responseId set")
    {
        REQUIRE(message->payloadType() == indiemotion::net::PayloadType::Acknowledge);
        REQUIRE(message->id() == indiemotion::net::Identifier("someid"));
        REQUIRE(message->inResponseToId().value() == indiemotion::net::Identifier("someresponseid"));
    }
}