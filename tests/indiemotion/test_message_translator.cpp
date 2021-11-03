#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/motion/mode.hpp>
#include <indiemotion/net/acknowledge.hpp>
#include <indiemotion/net/camera.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/motion.hpp>
#include <indiemotion/net/protobuf.hpp>
#include <indiemotion/net/translator.hpp>

TEST_CASE("Translate Acknowledge Messages")
{
    auto translator = indiemotion::net::MessageTranslator();

    auto payload = std::make_unique<indiemotion::net::Acknowledge>();
    auto message = indiemotion::net::createMessage(
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

TEST_CASE("Translate GetCameraList Messages (Not Supported)")
{
    auto translator = indiemotion::net::MessageTranslator();
    auto payload = std::make_unique<indiemotion::net::GetCameraList>();
    auto message = indiemotion::net::createMessage(std::move(payload));

    SUBCASE("thorws when trying to translate")
    {
        REQUIRE_THROWS_AS(translator.translateMessage(std::move(message)), std::runtime_error);
    }
}

TEST_CASE("Translate CameraList Messages")
{
    auto translator = indiemotion::net::MessageTranslator();
    auto expectedCams = std::vector<indiemotion::cameras::Camera>{
        indiemotion::cameras::Camera("cam1"),
        indiemotion::cameras::Camera("cam2"),
        indiemotion::cameras::Camera("cam3"),
    };
    auto payload = std::make_unique<indiemotion::net::CameraList>(expectedCams);
    auto message = indiemotion::net::createMessage("inResponseToID", std::move(payload));
    auto p = translator.translateMessage(std::move(message));

    SUBCASE("protobuf message has message header")
    {
        REQUIRE(p.header().has_responseid());
        REQUIRE(p.header().responseid() == "inResponseToID");
    }

    SUBCASE("protobuf message is filled with cameras")
    {
        REQUIRE(p.has_camera_list());
        REQUIRE(p.camera_list().camera_size() == 3);

        REQUIRE((p.camera_list().camera()[0]).id() == "cam1");
        REQUIRE((p.camera_list().camera()[1]).id() == "cam2");
        REQUIRE((p.camera_list().camera()[2]).id() == "cam3");
    }
}

TEST_CASE("Translate Motion Active Mode Messages")
{
    auto translator = indiemotion::net::MessageTranslator();
    auto payload = std::make_unique<indiemotion::net::MotionActiveMode>(
        indiemotion::motion::MotionMode::Live);
    auto message = indiemotion::net::createMessage("inResponseToID", std::move(payload));
    auto p = translator.translateMessage(std::move(message));

    SUBCASE("protobuf message has message header")
    {
        REQUIRE(p.header().has_responseid());
        REQUIRE(p.header().responseid() == "inResponseToID");
    }

    SUBCASE("protobuf payload is filled")
    {
        REQUIRE(p.has_motion_active_mode());
        REQUIRE(p.motion_active_mode().mode() == indiemotion::protobuf::payloads::v1::MotionMode::Live);
    }
}