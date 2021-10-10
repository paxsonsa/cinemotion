// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* test_motion_modes.cpp */
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/session/motion_state.hpp>

using namespace indiemotion;

SCENARIO("When Motion mode is off, setting off is ok")
{
    GIVEN("fresh context set to 'off'")
    {
        auto mode = motion::ModeController::create();

        WHEN("set mode to off")
        {
            mode->off();

            THEN("the current mode should be off")
            {
                REQUIRE(mode->current() == motion::Mode::Off);
            }
        }
    }
}

SCENARIO("When motion mode is off, setting live is ok")
{
    GIVEN("fresh context set to off")
    {
        auto mode = motion::ModeController::create();

        WHEN("set mode to live")
        {
            mode->live();

            THEN("the current mode should be live")
            {
                REQUIRE(mode->current() == motion::Mode::Live);
            }
        }
    }
}

SCENARIO("When motion mode is off, setting recording is ok")
{
    GIVEN("fresh context set to off")
    {
        auto mode = motion::ModeController::create();

        WHEN("set mode to recording")
        {
            mode->record();

            THEN("the current mode should be recording")
            {
                REQUIRE(mode->current() == motion::Mode::Recording);
            }
        }
    }
}
