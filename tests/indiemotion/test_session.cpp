#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/services/session_service.hpp>

using namespace indiemotion;

struct DummyDelegate: SessionDelegate
{
	bool initialized = false;
	std::string name = "";
	bool shutdown = false;

	void session_updated(const std::shared_ptr<const SessionContext>& session) override
	{
		initialized = session->initialized;
		name = session->name;
		shutdown = session->shutdown;
	}
};

TEST_CASE("Test Session Lifecycle")
{
	auto delegate = std::make_shared<DummyDelegate>();
	auto ctx = indiemotion::make_context();
	auto session = indiemotion::Session(ctx, delegate);

	session.initialize("testname");
	REQUIRE(ctx->session->initialized == true);
	REQUIRE(ctx->session->name == "testname");
	REQUIRE(delegate->initialized == true);
	REQUIRE(delegate->name == "testname");

	session.shutdown();
	REQUIRE(ctx->session->shutdown == true);
	REQUIRE(delegate->shutdown == true);
}
