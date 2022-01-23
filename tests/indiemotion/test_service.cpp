#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/service.hpp>

using namespace indiemotion;

struct DummyDelegate: SessionDelegate
{
	SessionContext session_ctx;

	void session_updated(std::shared_ptr<const SessionContext> session) override
	{
		session_ctx = *session;
	}
};

struct DummyDispatcher : NetMessageDispatcher {
	std::vector<Message> messages{};

	void dispatch(Message &&message) {
		messages.push_back(std::move(message));
	}
};

SCENARIO("Initializing the Session")
{
	GIVEN("a new controller object") {

		auto delegate = std::make_shared<DummyDelegate>();
		DelegateTable d_table;
		d_table.session_delegate = delegate;

		auto context = make_context();
		auto session = std::make_shared<SessionController>(context, d_table);
		auto dispatcher = std::make_shared<DummyDispatcher>();
		auto service = Service(dispatcher, session);

		Message message;
		auto payload = message.mutable_initialize_session();
		auto properties = payload->mutable_session_info();
		properties->set_api_version("1.0");
		properties->set_session_name("some_id");

		WHEN("start description is processed") {
			service.process_message(std::move(message));

			THEN("session controller status should be activated") {
				REQUIRE(delegate->session_ctx.initialized);
			}

			THEN("session controller name should be set") {
				REQUIRE(delegate->session_ctx.name == "some_id");
			}

			// TODO Confirm Emission of Scene and Motion Info.
		}
	}
}