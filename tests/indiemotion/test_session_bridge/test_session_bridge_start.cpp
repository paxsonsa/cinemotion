#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/net/message.hpp>

using namespace indiemotion;

struct DummyDispatcher : NetMessageDispatcher {
    std::vector<Message> messages{};

    void dispatch(Message &&message) {
        messages.push_back(std::move(message));
    }
};

struct DummyDelegate: SessionControllerDelegate
{
    bool sessionWillStartCalled = false;
    bool sessionDidStartCalled = false;

    void will_start_session() override {
        sessionWillStartCalled = true;
    }

    void did_start_session() override
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

        Message message;
        auto payload = message.mutable_session_start();
        auto properties = payload->mutable_session_properties();
        properties->set_api_version(SessionBridge::APIVersion);
        properties->set_session_id("some_id");

        WHEN("start description is processed") {
            bridge.process_message(std::move(message));

            THEN("No response should be returned") {
                REQUIRE(dispatcher->messages.size() == 0);
            }
            AND_THEN("session controller status should be activated") {
                REQUIRE(session->status() == SessionStatus::Initialized);
            }

            AND_THEN("session delegate's will_start_session() and did_start_session()")
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

        WHEN("start description is processed with unsupported version")
        {
            Message message;
            auto payload = message.mutable_session_start();
            auto properties = payload->mutable_session_properties();
            properties->set_api_version("99.9.999");
            properties->set_session_id("some_id");

            bridge.process_message(std::move(message));

            THEN("A session error description should be sent.")
            {
                REQUIRE(dispatcher->messages.size() == 1);
                auto response = dispatcher->messages[0];
                REQUIRE(response.has_error());
                auto error = response.error();
                REQUIRE(error.type() == "SessionAPIVersionNotSupportedError");
            }
        }
    }
}
