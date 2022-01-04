#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/motion/xform.hpp>

using namespace indiemotion;

SCENARIO("creating motion xform")
{
    GIVEN("a xform")
    {
        auto xform = MotionXForm::create(
            1.0f, 2.0f, 3.0f,
            4.0f, 5.0f, 6.0f);

        WHEN("it is moved")
        {
            auto newXform = std::move(xform);
            THEN("the object is the same")
            {
                REQUIRE(xform.translation.x == 0.0f);
                REQUIRE(xform.translation.y == 0.0f);
                REQUIRE(xform.translation.z == 0.0f);
                REQUIRE(xform.orientation.x == 0.0f);
                REQUIRE(xform.orientation.y == 0.0f);
                REQUIRE(xform.orientation.z == 0.0f);

                REQUIRE(newXform.translation.x == 1.0f);
                REQUIRE(newXform.translation.y == 2.0f);
                REQUIRE(newXform.translation.z == 3.0f);
                REQUIRE(newXform.orientation.x == 4.0f);
                REQUIRE(newXform.orientation.y == 5.0f);
                REQUIRE(newXform.orientation.z == 6.0f);
            }
        }
    }
}