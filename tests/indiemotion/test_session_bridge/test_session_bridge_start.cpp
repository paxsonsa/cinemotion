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

struct DummyDelegate: SessionControllerDelegate
{
    bool sessionWillStartCalled = false;
    bool sessionDidStartCalled = false;

    void sessionWillStart() override {
        sessionWillStartCalled = true;
    }

    void sessionDidStart() override
    {
        sessionDidStartCalled = true;
    }
};

SCENARIO("Starting the Session")
{
    GIVEN("a new controller object") {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);

        NetMessage message;
        auto payload = message.mutable_session_start();
        auto properties = payload->mutable_session_properties();
        properties->set_api_version(SessionBridge::APIVersion);
        properties->set_session_id("some_id");

        WHEN("start message is processed") {
            bridge.processMessage(std::move(message));

            THEN("No response should be returned") {
                REQUIRE(dispatcher->messages.size() == 0);
            }
            AND_THEN("session controller status should be activated") {
                REQUIRE(session->status() == SessionStatus::Initialized);
            }

            AND_THEN("session delegate's sessionWillStart() and sessionDidStart()")
            {
                REQUIRE(delegate->sessionWillStartCalled);
                REQUIRE(delegate->sessionDidStartCalled);
            }
        }
    }
}

SCENARIO("Starting the session with unsupported API version")
{
    GIVEN("a new controller")
    {
        auto session = std::make_shared<SessionController>();
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);

        WHEN("start message is processed with unsupported version")
        {
            NetMessage message;
            auto payload = message.mutable_session_start();
            auto properties = payload->mutable_session_properties();
            properties->set_api_version("99.9.999");
            properties->set_session_id("some_id");

            bridge.processMessage(std::move(message));

            THEN("A session error message should be sent.")
            {
                REQUIRE(dispatcher->messages.size() == 1);
                auto response = dispatcher->messages[0];
                REQUIRE(response.has_error());
                auto error = response.error();
                REQUIRE(error.type() == "SessionAPIVersionNotSupported");
            }
        }
    }
}
