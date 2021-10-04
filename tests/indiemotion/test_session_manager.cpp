// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp 

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/_common.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/messages/ack_message.hpp>

using namespace indiemotion;

SCENARIO("SessionManager Initialization")
{   
    GIVEN("a new manager object")
    {   
        auto manager = indiemotion::session::SessionManager();

        WHEN("manager.initalize() is called")
        {
            auto msg = manager.initialize();

            THEN("initialize() should have returned a message")
            {
                REQUIRE(msg);

                AND_THEN("the message should be a properly init message")
                {
                    REQUIRE(msg->getKind() == messages::Kind::InitSession);
                    REQUIRE(msg->requiresAck() == true);
                }
            }            
        }
    }

    GIVEN("an initialized session manager")
    {
        auto manager = session::SessionManager();
        auto msg = manager.initialize();
        auto id = msg->getId();
        WHEN("the manager processes an ACK for the init message")
        {
            auto ackMsg = std::make_unique<messages::types::AckMessage>(id);
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