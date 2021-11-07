#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/motion/xform.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/motion.hpp>
#include <indiemotion/net/translator.hpp>

TEST_CASE("Translate Motion Get Mode Messages Throws Exceptions")
{
    auto translator = indiemotion::net::MessageTranslator();
    auto payload = std::make_unique<indiemotion::net::MotionGetMode>();
    auto message = indiemotion::net::createMessage(std::move(payload));

    SUBCASE("translator should throw runtime error")
    {
        REQUIRE_THROWS_AS(translator.translateMessage(std::move(message)), std::runtime_error);
    }
}

TEST_CASE("Translate Motion Get Mode to Protobuf")
{
    auto translator = indiemotion::net::MessageTranslator();

    indiemotion::protobuf::messages::Message protobuf;
    auto header = protobuf.mutable_header();
    header->set_id("someid");
    protobuf.mutable_motion_get_mode();

    SUBCASE("returns message")
    {
        auto message = translator.translateProtobuf(std::move(protobuf));
        REQUIRE(message->payloadType() == indiemotion::net::PayloadType::MotionGetMode);
        REQUIRE(message->id() == indiemotion::net::Identifier("someid"));
    }
}