#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/motion/xform.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/motion.hpp>
#include <indiemotion/net/translator.hpp>

TEST_CASE("Translate MotionUpdateXForm Throws Exception")
{
    auto translator = indiemotion::net::MessageTranslator();
    auto xform = indiemotion::MotionXForm::create(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    auto payload = std::make_unique<indiemotion::net::MotionUpdateXForm>(xform);
    auto message = indiemotion::netMakeMessage(std::move(payload));

    SUBCASE("translator should throw runtime error")
    {
        REQUIRE_THROWS_AS(translator.translateMessage(std::move(message)), std::runtime_error);
    }
}

TEST_CASE("Translate Motion Xform Update to Protobuf")
{
    auto translator = indiemotion::net::MessageTranslator();

    indiemotion::protobuf::messages::Message protobuf;
    auto header = protobuf.mutable_header();
    header->set_id("someid");
    auto xform = protobuf.mutable_motion_xform();
    auto translation = xform->mutable_translation();
    translation->set_x(1.0);
    translation->set_y(2.0);
    translation->set_z(3.0);
    auto orientation = xform->mutable_orientation();
    orientation->set_x(4.0);
    orientation->set_y(5.0);
    orientation->set_z(6.0);

    SUBCASE("returns message")
    {
        auto message = translator.translateProtobuf(std::move(protobuf));
        REQUIRE(message->payloadType() == indiemotion::NetPayloadType::MotionUpdateXForm);
        REQUIRE(message->id() == indiemotion::NetIdentifier("someid"));

        auto inPayload = message->payloadPtrAs<indiemotion::net::MotionUpdateXForm>();
        REQUIRE(inPayload->xform.translation.x == 1.0);
        REQUIRE(inPayload->xform.translation.y == 2.0);
        REQUIRE(inPayload->xform.translation.z == 3.0);
        REQUIRE(inPayload->xform.orientation.x == 4.0);
        REQUIRE(inPayload->xform.orientation.y == 5.0);
        REQUIRE(inPayload->xform.orientation.z == 6.0);
    }
}