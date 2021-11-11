// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/net/dispatcher.hpp>
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

        WHEN("manager.start() is called") {
            bridge.start();
            auto response = std::move(dispatcher->messages[0]);

            THEN("start() should have returned a response") {
                REQUIRE(dispatcher->messages.size() == 1);
            }
            AND_THEN("the response should be a session start message") {
                REQUIRE(response.payload_case() == NetMessage::PayloadCase::kSessionStart);
            }
            AND_THEN("the response should be the session server info we expect") {
                auto payload = response.session_start();

                REQUIRE(payload.server_info().api_version() == SessionBridge::APIVersion);
                REQUIRE(payload.server_info().features() == 0);
            }
            AND_THEN("session controller status should be starting") {
                REQUIRE(session->status() == SessionStatus::Starting);
            }
        }
    }
}
SCENARIO("Activating the Session")
{
    GIVEN("an started session controller") {
        auto session = std::make_shared<SessionController>();
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);
        bridge.start();

        // Clear the messages so far
        dispatcher->messages.clear();

        WHEN("the client sends a session activate message") {
            auto message = netMakeMessage();
            auto payload = message.mutable_session_activate();
            payload->mutable_session_properties();

            bridge.processMessage(std::move(message));

            THEN("no message should be returned") {
                REQUIRE(dispatcher->messages.size() == 0);
            }

            AND_THEN("the session should be active") {
                REQUIRE(session->status() == SessionStatus::Activated);
            }
        }
    }
}
