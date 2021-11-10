#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/motion/mode.hpp>
#include <indiemotion/motion/xform.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/motion.hpp>
#include <indiemotion/net/translator.hpp>

TEST_CASE("Translate Motion Set Mode Messages Throws Exceptions")
{
    auto translator = indiemotion::net::MessageTranslator();
    auto payload = std::make_unique<indiemotion::net::MotionSetMode>(
        indiemotion::MotionMode::Live);
    auto message = indiemotion::netMakeMessage(std::move(payload));

    SUBCASE("translator should throw runtime error")
    {
        REQUIRE_THROWS_AS(translator.translateMessage(std::move(message)), std::runtime_error);
    }
}

TEST_CASE("Translate Motion Set Mode [Live] to Protobuf")
{
    auto translator = indiemotion::net::MessageTranslator();

    indiemotion::protobuf::messages::Message protobuf;
    auto header = protobuf.mutable_header();
    header->set_id("someid");

    SUBCASE("returns live message")
    {
        auto payload = protobuf.mutable_motion_set_mode();
        payload->set_mode(indiemotion::protobuf::payloads::v1::MotionMode::Live);

        auto message = translator.translateProtobuf(std::move(protobuf));
        REQUIRE(message->payloadType() == indiemotion::NetPayloadType::MotionSetMode);
        REQUIRE(message->id() == indiemotion::NetIdentifier("someid"));

        auto outPayload = message->payloadPtrAs<indiemotion::net::MotionSetMode>();
        REQUIRE(outPayload->mode == indiemotion::MotionMode::Live);
    }

    SUBCASE("returns recording message")
    {
        auto payload = protobuf.mutable_motion_set_mode();
        payload->set_mode(indiemotion::protobuf::payloads::v1::MotionMode::Recording);

        auto message = translator.translateProtobuf(std::move(protobuf));
        REQUIRE(message->payloadType() == indiemotion::NetPayloadType::MotionSetMode);
        REQUIRE(message->id() == indiemotion::NetIdentifier("someid"));

        auto outPayload = message->payloadPtrAs<indiemotion::net::MotionSetMode>();
        REQUIRE(outPayload->mode == indiemotion::MotionMode::Recording);
    }

    SUBCASE("returns off message")
    {
        auto payload = protobuf.mutable_motion_set_mode();
        payload->set_mode(indiemotion::protobuf::payloads::v1::MotionMode::Off);

        auto message = translator.translateProtobuf(std::move(protobuf));
        REQUIRE(message->payloadType() == indiemotion::NetPayloadType::MotionSetMode);
        REQUIRE(message->id() == indiemotion::NetIdentifier("someid"));

        auto outPayload = message->payloadPtrAs<indiemotion::net::MotionSetMode>();
        REQUIRE(outPayload->mode == indiemotion::MotionMode::Off);
    }
}