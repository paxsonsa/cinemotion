// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* test_motion_postion_message.hpp 


*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../fixtures/managerFixtures.hpp"
#include <doctest.h>

#include <indiemotion/_common.hpp>
// #include <indiemotion/errors.hpp>
#include <indiemotion/messages/messages.hpp>
// #include <indiemotion/responses/responses.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

SCENARIO("Inactive Session Fails to Update MotionXForm")
{
    GIVEN("a new session")
    {
        auto manager = session::SessionManager();

        WHEN("the session is inactive")
        {
            REQUIRE_FALSE(manager.session()->isActive());

            AND_WHEN("the manager tries to process a positoin update message")
            {
                auto position = indiemotion::motion::MotionXForm::zero();
                auto messagePtr = indiemotion::messages::motion::xform::Message::create(std::move(position));
                THEN("an inactive session error should be thrown")
                {
                    REQUIRE_THROWS_AS(manager.processMessage(std::move(messagePtr)), indiemotion::errors::SessionError);
                }
            }
        }
    }
}

SCENARIO("Updating motion xform on active session")
{

    class DummyDelegate : public session::SessionDelegate
    {
    public:
        std::unique_ptr<motion::MotionXFormView> xformView;
        int motionDidUpdateCalled = 0;

        void motionDidUpdate(std::unique_ptr<motion::MotionXFormView> xform)
        {
            xformView = std::move(xform);
            motionDidUpdateCalled += 1;
        }
    };

    GIVEN("an activated session")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto manager = session::SessionManager();
        manager.session()->bindDelegate(delegate);

        WHEN("the session is active")
        {
            manager.session()->activate();
            REQUIRE(manager.session()->isActive());

            AND_WHEN("the manager tries to process a position update message")
            {
                auto xform = indiemotion::motion::MotionXForm::zero();
                xform->translation->x = 1.0;
                xform->translation->y = 2.0;
                xform->translation->z = 3.0;
                xform->orientation->x = 4.0;
                xform->orientation->y = 5.0;
                xform->orientation->z = 6.0;
                auto messagePtr = indiemotion::messages::motion::xform::Message::create(std::move(xform));

                manager.processMessage(std::move(messagePtr));
                auto curXformView = manager.session()->motionView();
                THEN("the current xform should be updated")
                {
                    // TODO auto curXform = manager.session()->motionController()->currentXform();
                    REQUIRE(curXformView->translationX() == 1.0);
                    REQUIRE(curXformView->translationY() == 2.0);
                    REQUIRE(curXformView->translationZ() == 3.0);
                    REQUIRE(curXformView->orientationX() == 4.0);
                    REQUIRE(curXformView->orientationY() == 5.0);
                    REQUIRE(curXformView->orientationZ() == 6.0);
                }

                THEN("the delegate should have been called")
                {
                    REQUIRE(delegate->motionDidUpdateCalled == 1);
                    REQUIRE(delegate->xformView == curXformView);
                }
            }
        }
    }
}