// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* test_motion_modes.cpp */
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/session/motion_mode.hpp>

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
                REQUIRE(mode->current() == motion::ModeValue::Off);
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
                REQUIRE(mode->current() == motion::ModeValue::Live);
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
                REQUIRE(mode->current() == motion::ModeValue::Recording);
            }
        }
    }
}

SCENARIO("When motion mode is live, setting off is ok")
{
    GIVEN("context set to live")
    {
        auto mode = motion::ModeController::create();
        mode->live();
        REQUIRE(mode->current() == motion::ModeValue::Live);

        WHEN("set mode to off")
        {
            mode->off();

            THEN("the current mode should be off")
            {
                REQUIRE(mode->current() == motion::ModeValue::Off);
            }
        }
    }
}

SCENARIO("When motion mode is live, setting live is ok")
{
    GIVEN("context set to live")
    {
        auto mode = motion::ModeController::create();
        mode->live();
        REQUIRE(mode->current() == motion::ModeValue::Live);

        WHEN("set mode to live")
        {
            mode->live();

            THEN("the current mode should stall Live")
            {
                REQUIRE(mode->current() == motion::ModeValue::Live);
            }
        }
    }
}

SCENARIO("When motion mode is live, setting record is ok")
{
    GIVEN("context set to live")
    {
        auto mode = motion::ModeController::create();
        mode->live();
        REQUIRE(mode->current() == motion::ModeValue::Live);

        WHEN("set mode to recording")
        {
            mode->record();

            THEN("the current mode should be recording")
            {
                REQUIRE(mode->current() == motion::ModeValue::Recording);
            }
        }
    }
}

SCENARIO("When motion mode is recording, setting off is ok")
{
    GIVEN("context set to recording")
    {
        auto mode = motion::ModeController::create();
        mode->record();
        REQUIRE(mode->current() == motion::ModeValue::Recording);

        WHEN("set mode to off")
        {
            mode->off();

            THEN("the current mode should be off")
            {
                REQUIRE(mode->current() == motion::ModeValue::Off);
            }
        }
    }
}

SCENARIO("When motion mode is recording, setting live is ok")
{
    GIVEN("context set to recording")
    {
        auto mode = motion::ModeController::create();
        mode->record();
        REQUIRE(mode->current() == motion::ModeValue::Recording);

        WHEN("live()")
        {
            mode->live();

            THEN("the current mode should be live")
            {
                REQUIRE(mode->current() == motion::ModeValue::Live);
            }
        }
    }
}

SCENARIO("When motion mode is recording, setting record() is ok")
{
    GIVEN("context set to recording")
    {
        auto mode = motion::ModeController::create();
        mode->record();
        REQUIRE(mode->current() == motion::ModeValue::Recording);

        WHEN("record()")
        {
            mode->record();

            THEN("the current mode should be Recording")
            {
                REQUIRE(mode->current() == motion::ModeValue::Recording);
            }
        }
    }
}

SCENARIO("Recording mode is recording")
{
    GIVEN("a new controller")
    {
        auto mode = motion::ModeController::create();

        WHEN("record() is called")
        {
            mode->record();

            THEN("isRecording() should be true")
            {
                REQUIRE(mode->isRecording());
            }

            AND_THEN("isCapturingMotion() should be true")
            {
                REQUIRE(mode->isCapturingMotion());
            }
        }
    }
}

SCENARIO("Live mode is not recording but capturing motion")
{
    GIVEN("a new controller")
    {
        auto mode = motion::ModeController::create();

        WHEN("live() is called")
        {
            mode->live();

            THEN("isRecording() should be false")
            {
                REQUIRE_FALSE(mode->isRecording());
            }

            AND_THEN("isCapturingMotion() should be true")
            {
                REQUIRE(mode->isCapturingMotion());
            }
        }
    }
}

SCENARIO("Off mode is not recording and not capturing motion")
{
    GIVEN("a new controller")
    {
        auto mode = motion::ModeController::create();

        WHEN("off() is called")
        {
            mode->off();

            THEN("isRecording() should be false")
            {
                REQUIRE_FALSE(mode->isRecording());
            }

            AND_THEN("isCapturingMotion() should be false")
            {
                REQUIRE_FALSE(mode->isCapturingMotion());
            }
        }
    }
}