// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/errors.hpp>
#include <indiemotion/messages/messages.hpp>
#include <indiemotion/responses/responses.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

SCENARIO("Initializing the Session")
{
    GIVEN("a new manager object")
    {
        auto manager = indiemotion::session::SessionManager();

        WHEN("manager.initalize() is called")
        {
            auto msg = manager.initialize();

            THEN("initialize() should have returned a response")
            {
                REQUIRE(msg);

                AND_THEN("the message should be a properly init message")
                {
                    REQUIRE(msg->kind() == responses::Kind::InitSession);
                    REQUIRE(msg->needsAcknowledgment() == true);
                }
            }
        }
    }

    GIVEN("an initialized session manager")
    {
        auto manager = session::SessionManager();
        auto msg = manager.initialize();
        auto id = msg->id();
        WHEN("the manager processes an ACK for the init message")
        {
            auto ackMsg = std::make_unique<messages::acknowledge::Message>(id);
            auto noMsg = manager.processMessage(std::move(ackMsg));

            THEN("no message should be returned")
            {
                REQUIRE_FALSE(noMsg.has_value());
            }

            THEN("the session should be in an activated status")
            {
                REQUIRE(manager.session()->status() == session::state::SessionStatus::Active);
            }
        }
    }
}
