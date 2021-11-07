#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/motion/mode.hpp>
#include <indiemotion/motion/xform.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/motion.hpp>
#include <indiemotion/net/protobuf.hpp>
#include <indiemotion/net/translator.hpp>

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