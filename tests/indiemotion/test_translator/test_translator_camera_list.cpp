#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/motion/mode.hpp>
#include <indiemotion/motion/xform.hpp>
#include <indiemotion/net/camera.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/motion.hpp>
#include <indiemotion/net/protobuf.hpp>
#include <indiemotion/net/translator.hpp>

TEST_CASE("Translate CameraList Messages")
{
    auto translator = indiemotion::net::MessageTranslator();
    auto expectedCams = std::vector<indiemotion::cameras::Camera>{
        indiemotion::cameras::Camera("cam1"),
        indiemotion::cameras::Camera("cam2"),
        indiemotion::cameras::Camera("cam3"),
    };
    auto payload = std::make_unique<indiemotion::NetCameraList>(expectedCams);
    auto message = indiemotion::netMakeMessageWithResponseID("inResponseToID", std::move(payload));
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