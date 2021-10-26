// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/errors.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

SCENARIO("Initializing the Session")
{
    GIVEN("a new manager object")
    {
        auto session = std::make_shared<session::Session>();
        auto bridge = indiemotion::session::SessionBridge(session);

        WHEN("manager.initalize() is called")
        {
            auto response = bridge.initialize();

            THEN("initialize() should have returned a response")
            {
                REQUIRE(response);

                AND_THEN("the message should be a properly init message")
                {
                    REQUIRE(response->payloadType() == indiemotion::net::PayloadType::SessionInitilization);
                }
            }
        }
    }

    // GIVEN("an initialized session manager")
    // {
    //     auto manager = session::SessionManager();
    //     auto response = manager.initialize();
    //     auto responseId = response->id();
    //     WHEN("the manager processes an ACK for the init message")
    //     {
    //         auto payload = messages::acknowledge::Payload::create(true, "");
    //         auto message = messages::base::createMessage(responseId, std::move(payload));
    //         auto noMsg = manager.processMessage(std::move(message));

    //         THEN("no message should be returned")
    //         {
    //             REQUIRE_FALSE(noMsg.has_value());
    //         }

    //         THEN("the session should be in an activated status")
    //         {
    //             REQUIRE(manager.session()->status() == session::state::SessionStatus::Active);
    //         }
    //     }
    // }
}
