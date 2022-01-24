#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/services/session_service.hpp>

using namespace indiemotion;

struct DummyDelegate: MotionDelegate
{
	MotionStatus status;

	void motion_updated(std::shared_ptr<const MotionContext> motion) override
	{
		status = motion->status;
	}
};

TEST_CASE("Test Motion Lifecycle")
{
	auto delegate = std::make_shared<DummyDelegate>();
	auto ctx = indiemotion::make_context();
	auto motion = indiemotion::MotionService(ctx, delegate);

	REQUIRE(motion.status() == MotionStatus::Idle);
	REQUIRE(delegate->status == MotionStatus::Idle);

	motion.status(MotionStatus::Live);
	REQUIRE(delegate->status == MotionStatus::Live);
}
