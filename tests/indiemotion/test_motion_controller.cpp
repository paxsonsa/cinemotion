// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* test_motion_controller.hpp */
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/motion.hpp>

SCENARIO("Motion transforms are updated")
{
    class DummyDelegate : public indiemotion::motion::MotionDelegate
    {
    public:
        std::shared_ptr<indiemotion::motion::MotionXForm> currentXform;
        int didUpdateCalled = 0;
        void didUpdate(std::shared_ptr<indiemotion::motion::MotionXForm> xform)
        {
            currentXform = xform;
            didUpdateCalled += 1;
        }
    };
    GIVEN("a motion controller with a delegate")
    {
        auto delegatePtr = std::make_shared<DummyDelegate>();
        auto controller = indiemotion::motion::MotionController(delegatePtr);

        WHEN("we update the current transform")
        {
            auto xform = indiemotion::motion::MotionXForm::create(
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0);

            controller.update(std::move(xform));

            THEN("expect xform to be passed to delegate")
            {
                REQUIRE(delegatePtr->didUpdateCalled == 1);
                REQUIRE(delegatePtr->currentXform->translation.x == 1.0);
                REQUIRE(delegatePtr->currentXform->translation.y == 2.0);
                REQUIRE(delegatePtr->currentXform->translation.z == 3.0);
                REQUIRE(delegatePtr->currentXform->orientation.x == 4.0);
                REQUIRE(delegatePtr->currentXform->orientation.y == 5.0);
                REQUIRE(delegatePtr->currentXform->orientation.z == 6.0);
            }
        }
    }
}