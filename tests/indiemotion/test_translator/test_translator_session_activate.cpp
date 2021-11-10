#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/net/session.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/translator.hpp>
#include <indiemotion/net/protobuf.hpp>
#include <indiemotion/session/properties.hpp>

TEST_CASE("Translate Session Activate Messages")
{
    struct DummyPayload : public indiemotion::NetPayload_T {
        indiemotion::NetPayloadType type() const override {
            return indiemotion::NetPayloadType::SessionActivate;
        }
    };
    auto translator = indiemotion::net::MessageTranslator();
    auto payload = std::make_unique<DummyPayload>();
    auto message = indiemotion::netMakeMessageWithId(
        indiemotion::NetIdentifier("someID"),
        std::move(payload));

    SUBCASE("should fail") {
        REQUIRE_THROWS_AS(translator.translateMessage(std::move(message)),
                          std::runtime_error);
    }
}

TEST_CASE("Translate Acknowledge Protobuf")
{
    auto translator = indiemotion::net::MessageTranslator();

    indiemotion::protobuf::messages::Message protobuf;
    auto header = protobuf.mutable_header();
    header->set_id("someID");
    auto payload = protobuf.mutable_session_activate();
    payload->mutable_sessionproperties();

    auto message = translator.translateProtobuf(std::move(protobuf));

    SUBCASE("protobuf message has responseId set") {
        REQUIRE(message->payloadType() == indiemotion::NetPayloadType::SessionActivate);
        REQUIRE(message->id() == indiemotion::NetIdentifier("someID"));
    }
}
