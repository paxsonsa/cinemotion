#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/net/session.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/translator.hpp>
#include <indiemotion/net/protobuf.hpp>
#include <indiemotion/session/server_info.hpp>

TEST_CASE("Translate Session Start Messages")
{
    auto translator = indiemotion::net::MessageTranslator();
    auto info = indiemotion::SessionServerInfo(
        "1.0.0",
        indiemotion::SessionServerFeatureSet(
            indiemotion::SessionServerFeature::VideoStreaming
        )
    );
    auto payload = std::make_unique<indiemotion::NetSessionStart>(info);
    auto message = indiemotion::netMakeMessageWithId(indiemotion::NetIdentifier("someID"),
                                                     std::move(payload));

    auto protobuf = translator.translateMessage(std::move(message));
    auto translatedInfo = protobuf.session_start().serverinfo();

    REQUIRE(protobuf.header().id() == "someID");
    REQUIRE(translatedInfo.apiversion() == "1.0.0");
    REQUIRE(translatedInfo.features() == (uint32_t)indiemotion::SessionServerFeatureSet(
        indiemotion::SessionServerFeature::VideoStreaming
    ));
}

TEST_CASE("Translate Acknowledge Protobuf")
{
    auto translator = indiemotion::net::MessageTranslator();

    indiemotion::protobuf::messages::Message protobuf;
    protobuf.mutable_session_start();

    REQUIRE_THROWS_AS(translator.translateProtobuf(std::move(protobuf)), std::runtime_error);
}
