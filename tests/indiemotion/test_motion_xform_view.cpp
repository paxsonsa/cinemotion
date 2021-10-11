#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>

#include <indiemotion/motion.hpp>
#include <indiemotion/session.hpp>

SCENARIO("ensure that the motion view updates as the motion xform changes")
{
    GIVEN("a session and its returned motion view")
    {
        auto session = indiemotion::session::Session();
        session.activate();
        auto view = session.motionView();

        WHEN("the motion xform is updated")
        {
            // TODO MotionView: add update xform membver function
            auto xformPtr = indiemotion::motion::MotionXForm::create(
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
            session.update(std::move(xformPtr));

            THEN("the motion view should have been updated")
            {
                REQUIRE(view->translation().x == 1.0);
                REQUIRE(view->translation().x == 2.0);
                REQUIRE(view->translation().x == 3.0);
                REQUIRE(view->orientation().x == 4.0);
                REQUIRE(view->orientation().x == 5.0);
                REQUIRE(view->orientation().x == 6.0);
            }
        }
    }
}