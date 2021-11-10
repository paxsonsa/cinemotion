#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/translator.hpp>
#include <indiemotion/net/protobuf.hpp>
#include <indiemotion/net/session.hpp>

TEST_CASE("Translate Session Shutdown Messages")
{
    auto translator = indiemotion::net::MessageTranslator();

    auto payload = std::make_unique<indiemotion::NetSessionShutdown>();
    auto message = indiemotion::netMakeMessage(std::move(payload));
    auto protobuf = translator.translateMessage(std::move(message));

    SUBCASE("protobuf message has acknowledge payload")
    {
        REQUIRE(protobuf.has_session_shutdown());
    }
}

TEST_CASE("Translate Session Shutdown Protobuf")
{
    auto translator = indiemotion::net::MessageTranslator();

    indiemotion::protobuf::messages::Message protobuf;
    auto header = protobuf.mutable_header();
    header->set_id("someID");
    protobuf.mutable_session_shutdown();

    std::shared_ptr<indiemotion::NetMessage> message = translator.translateProtobuf(std::move(protobuf));

    SUBCASE("message is expected types")
    {
        REQUIRE(message->payloadType() == indiemotion::NetPayloadType::SessionShutdown);
        REQUIRE(message->id() == indiemotion::NetIdentifier("someID"));
    }
}