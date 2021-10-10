// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp 

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/_common.hpp>
#include <indiemotion/errors.hpp>
#include <indiemotion/messages/messages.hpp>
#include <indiemotion/responses/responses.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

SCENARIO("Initial Motion Mode should be Off")
{
    class DummyDelegate : public session::SessionDelegate
    {
    };

    GIVEN("a new active session")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto manager = session::SessionManager();
        manager.session()->bindDelegate(delegate);
        manager.session()->activate();

        WHEN("the client sends a get mode message")
        {
            auto msg = std::make_unique<messages::motion::get_mode::Message>();
            auto opt_response = manager.processMessage(std::move(msg));

            THEN("the response should return the current motion mode on the session")
            {
                REQUIRE(opt_response);
                auto response = std::move(opt_response.value());
                REQUIRE(response->kind() == responses::Kind::MotionCurrentMode);
                REQUIRE(response->needsAcknowledgment() == false);

                auto curModeMsg = static_unique_pointer_cast<responses::motion::current_mode::Response>(std::move(response));
                REQUIRE(curModeMsg->mode() == manager.session()->motionMode());
            }
        }
    }
}