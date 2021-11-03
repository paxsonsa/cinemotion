#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/net/acknowledge.hpp>
#include <indiemotion/net/message.hpp>
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