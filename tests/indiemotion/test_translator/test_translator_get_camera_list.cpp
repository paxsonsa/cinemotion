#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/motion/mode.hpp>
#include <indiemotion/motion/xform.hpp>
#include <indiemotion/net/acknowledge.hpp>
#include <indiemotion/net/camera.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/motion.hpp>
#include <indiemotion/net/protobuf.hpp>
#include <indiemotion/net/translator.hpp>

TEST_CASE("Translate GetCameraList Messages Throws Exceptions")
{
    auto translator = indiemotion::net::MessageTranslator();
    auto payload = std::make_unique<indiemotion::net::GetCameraList>();
    auto message = indiemotion::netMakeMessage(std::move(payload));

    SUBCASE("throws when trying to translate")
    {
        REQUIRE_THROWS_AS(translator.translateMessage(std::move(message)), std::runtime_error);
    }
}

TEST_CASE("Translate GetCameraList Messages Throws Exceptions")
{
    auto translator = indiemotion::net::MessageTranslator();

    indiemotion::protobuf::messages::Message protobuf;
    auto header = protobuf.mutable_header();
    header->set_id("someid");
    protobuf.mutable_get_camera_list();

    SUBCASE("returns GetCameraList message")
    {
        auto message = translator.translateProtobuf(std::move(protobuf));
        REQUIRE(message->payloadType() == indiemotion::NetPayloadType::GetCameraList);
        REQUIRE(message->id() == indiemotion::NetIdentifier("someid"));
    }
}
