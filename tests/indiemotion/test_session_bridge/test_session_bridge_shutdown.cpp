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

struct DummyDelegate : SessionControllerDelegate{
    bool sessionWillShutdownCalled = false;

    void will_shutdown_session() //override
    {
        sessionWillShutdownCalled = true;
    }
};

SCENARIO("signalling session shutdown successfully")
{
    GIVEN("an activated session controller")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);
        session->initialize();

        WHEN("the client signals a session shutdown")
        {
            auto message = net_make_message();
            message.mutable_session_shutdown();
            bridge.process_message(std::move(message));

            THEN("then session status should be moved to off")
            {
                REQUIRE(session->status() == SessionStatus::Offline);
            }

            THEN("the session delegate is called")
            {
                REQUIRE(delegate->sessionWillShutdownCalled);
            }
        }
    }
}
