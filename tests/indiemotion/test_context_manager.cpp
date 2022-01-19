#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/context.hpp>
#include <indiemotion/context_manager.hpp>

TEST_CASE("Test ContextView Management")
{
	auto context = indiemotion::Context::new_default_context();
	auto manager = indiemotion::ContextManager(std::move(context));

	auto xform = indiemotion::EulerXForm(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
	auto info = std::make_unique<indiemotion::DeviceInfo>();
	info->current_xform = xform;

	auto c = manager.mutable_context();
	c->device_info(std::move(info));
	REQUIRE(xform == manager.context()->device_info()->current_xform);
}