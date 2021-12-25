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

struct DummyDelegate: SessionControllerDelegate {
};

SCENARIO("Send a message without a payload case")
{
    GIVEN("a new controller object") {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);

        NetMessage message;

        WHEN("message without a payload is processed")
		{
			bridge.process_message(std::move(message));

			THEN("A 'malformed message' error message should be returned should be returned")
			{
				REQUIRE(dispatcher->messages.size() == 1);
				auto response = dispatcher->messages[0];
				REQUIRE(response.has_error());
				auto error = response.error();
				REQUIRE(error.type() == "MalformedMessageError");
			}
		}
	}
}
