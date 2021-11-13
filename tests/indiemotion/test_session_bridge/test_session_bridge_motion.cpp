// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/net/message.hpp>

using namespace indiemotion;

struct DummyDispatcher : NetMessageDispatcher {
    std::vector<NetMessage> messages{};

    void dispatch(NetMessage &&message) {
        messages.push_back(std::move(message));
    }
};

struct DummyDelegate : SessionControllerDelegate
{
    bool wasMotionModeDidUpdateCalled = false;
    MotionMode mode = MotionMode::Off;

    void didMotionSetMode(MotionMode m) override
    {
        wasMotionModeDidUpdateCalled = true;
        mode = m;
    }
};

SCENARIO("Set Motion Mode Successfully")
{
    GIVEN("an activated session controller")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);
        session->setStatus(SessionStatus::Activated);

        WHEN("bridge processes set motion mode=live message")
        {
            auto message = netMakeMessage();
            auto payload = message.mutable_motion_set_mode();
            payload->set_mode(netPayloadsV1::MotionMode::Live);
            bridge.processMessage(std::move(message));

            REQUIRE_FALSE(dispatcher->messages.size() > 0);

            THEN("the motion mode should be updated")
            {
                REQUIRE(session->currentMotionMode() == MotionMode::Live);
            }

            THEN("the delegates motion mode did update")
            {
                REQUIRE(delegate->wasMotionModeDidUpdateCalled);
                REQUIRE(delegate->mode == MotionMode::Live);
            }
        }

        WHEN("bridge processes set motion mode=recording message")
        {
            auto message = netMakeMessage();
            auto payload = message.mutable_motion_set_mode();
            payload->set_mode(netPayloadsV1::MotionMode::Recording);
            bridge.processMessage(std::move(message));

            REQUIRE_FALSE(dispatcher->messages.size() > 0);

            THEN("the motion mode should be updated")
            {
                REQUIRE(session->currentMotionMode() == MotionMode::Recording);
            }

            THEN("the delegates motion mode did update")
            {
                REQUIRE(delegate->wasMotionModeDidUpdateCalled);
                REQUIRE(delegate->mode == MotionMode::Recording);
            }
        }

        WHEN("bridge processes set motion mode=off message")
        {
            auto message = netMakeMessage();
            auto payload = message.mutable_motion_set_mode();
            payload->set_mode(netPayloadsV1::MotionMode::Off);
            bridge.processMessage(std::move(message));

            REQUIRE_FALSE(dispatcher->messages.size() > 0);

            THEN("the motion mode should be updated")
            {
                REQUIRE(session->currentMotionMode() == MotionMode::Off);
            }

            THEN("the delegates motion mode did update")
            {
                REQUIRE(delegate->wasMotionModeDidUpdateCalled);
                REQUIRE(delegate->mode == MotionMode::Off);
            }
        }
    }
}

