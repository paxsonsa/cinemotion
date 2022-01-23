#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/controller.hpp>

using namespace indiemotion;

struct DummyDelegate: MotionDelegate
{
	MotionXForm xform;
	MotionStatus status;

	void motion_updated(std::shared_ptr<const MotionContext> motion) override
	{
		status = motion->status;
		xform = motion->current_xform;
	}
};

TEST_CASE("Test Motion Lifecycle")
{
	auto delegate = std::make_shared<DummyDelegate>();
	auto ctx = indiemotion::make_context();
	auto motion = indiemotion::MotionManager(ctx, delegate);

	REQUIRE(motion.status() == MotionStatus::Idle);
	REQUIRE(delegate->status == MotionStatus::Idle);

	motion.status(MotionStatus::Live);
	REQUIRE(delegate->status == MotionStatus::Live);

	auto xform = MotionXForm::create(1,2,3,4,5,6);
	motion.update_xform(std::move(xform));
	REQUIRE(delegate->xform == MotionXForm::create(1,2,3,4,5,6));
}
