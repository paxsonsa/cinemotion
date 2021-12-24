#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/logging.hpp>

SCENARIO("Returning logger parent names")
{
    GIVEN("com.indiemotion.session.bridge")
    {
        auto name = "com.indiemotion.session.bridge";
        WHEN("listing the parent names")
        {
            auto loggerNames = indiemotion::logging::_list_parent_names(name);

            THEN("the proper list of logger names should be returned")
            {
                std::vector<std::string> expected{
                    "com.indiemotion.session.bridge",
                    "com.indiemotion.session",
                    "com.indiemotion",
                };

                std::cout << "expected: { ";
                for (auto i : expected)
                    std::cout << i << ' ';
                std::cout << " } \n";

                std::cout << "actual: { ";
                for (auto i : loggerNames)
                    std::cout << i << ' ';
                std::cout << " } \n";
                // std::cout << "actual: " << loggerNames << "\n";
                REQUIRE(expected == loggerNames);
            }
        }
    }
}