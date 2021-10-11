// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* test_motion_postion_message.hpp 


*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../fixtures/managerFixtures.hpp"
#include <doctest.h>

#include <indiemotion/_common.hpp>
// #include <indiemotion/errors.hpp>
#include <indiemotion/messages/messages.hpp>
// #include <indiemotion/responses/responses.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

SCENARIO("Inactive Session Fails to Update MotionXForm")
{
    GIVEN("a new session")
    {
        auto manager = session::SessionManager();

        WHEN("the session is inactive")
        {
            REQUIRE_FALSE(manager.session()->isActive());

            AND_WHEN("the manager tries to process a positoin update message")
            {
                auto position = indiemotion::motion::MotionXForm::zero();
                auto messagePtr = indiemotion::messages::motion::xform::Message::create(std::move(position));
                THEN("an inactive session error should be thrown")
                {
                    REQUIRE_THROWS_AS(manager.processMessage(std::move(messagePtr)), indiemotion::errors::SessionError);
                }
            }
        }
    }
}