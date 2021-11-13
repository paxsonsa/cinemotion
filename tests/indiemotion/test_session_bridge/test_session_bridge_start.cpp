// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
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

SCENARIO("Starting the Session")
{
    GIVEN("a new controller object") {
        auto session = std::make_shared<SessionController>();
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);

        NetMessage message;
        auto payload = message.mutable_session_start();
        auto properties = payload->mutable_session_properties();
        properties->set_api_version("1.0.0");
        properties->set_session_id("some_id");

        WHEN("start message is processed") {
            bridge.processMessage(std::move(message));

            THEN("No response should be returned") {
                REQUIRE(dispatcher->messages.size() == 0);
            }
            AND_THEN("session controller status should be activated") {
                REQUIRE(session->status() == SessionStatus::Activated);
            }
        }
    }
}
