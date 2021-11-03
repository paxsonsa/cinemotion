#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/net/acknowledge.hpp>
#include <indiemotion/net/camera.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/protobuf.hpp>
#include <indiemotion/net/translator.hpp>

SCENARIO("Translate Acknowledge Messages")
{
    GIVEN("A message translator")
    {
        auto translator = indiemotion::net::MessageTranslator();

        WHEN("translating an acknowledgement message")
        {
            auto payload = std::make_unique<indiemotion::net::Acknowledge>();
            auto message = indiemotion::net::createMessage(
                indiemotion::net::Identifier("somemessageID"),
                std::move(payload));
            auto protobuf = translator.translateMessage(std::move(message));

            THEN("protobuf message has acknowledge payload")
            {
                REQUIRE(protobuf.has_acknowledge());
            }

            THEN("protobuf message has responseId set")
            {
                REQUIRE(protobuf.header().has_responseid());
                REQUIRE(protobuf.header().responseid() == "somemessageID");
            }
        }
    }
}

SCENARIO("Translate GetCameraList Messages (Not Supported)")
{
    GIVEN("A message translator")
    {
        auto translator = indiemotion::net::MessageTranslator();

        WHEN("translating the message")
        {
            auto payload = std::make_unique<indiemotion::net::GetCameraList>();
            auto message = indiemotion::net::createMessage(std::move(payload));

            THEN("thorws when trying to translate")
            {
                REQUIRE_THROWS_AS(translator.translateMessage(std::move(message)), std::runtime_error);
            }
        }
    }
}

SCENARIO("Translate CameraList Messages")
{
    GIVEN("A message translator")
    {
        auto translator = indiemotion::net::MessageTranslator();

        WHEN("translating the message")
        {
            auto expectedCams = std::vector<indiemotion::cameras::Camera>{
                indiemotion::cameras::Camera("cam1"),
                indiemotion::cameras::Camera("cam2"),
                indiemotion::cameras::Camera("cam3"),
            };
            auto payload = std::make_unique<indiemotion::net::CameraList>(expectedCams);
            auto message = indiemotion::net::createMessage("inResponseToID", std::move(payload));

            auto p = translator.translateMessage(std::move(message));

            THEN("protobuf message has message header")
            {
                REQUIRE(p.header().has_responseid());
                REQUIRE(p.header().responseid() == "inResponseToID");
            }

            THEN("protobuf message is filled with cameras")
            {
                REQUIRE(p.has_camera_list());
                REQUIRE(p.camera_list().camera_size() == 3);

                REQUIRE((p.camera_list().camera()[0]).id() == "cam1");
                REQUIRE((p.camera_list().camera()[1]).id() == "cam2");
                REQUIRE((p.camera_list().camera()[2]).id() == "cam3");
            }
        }
    }
}