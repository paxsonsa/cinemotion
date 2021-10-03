// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* test_session.cpp 

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/_common.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/messages.hpp>

using namespace indiemotion;

TEST_SUITE("test session")
{
    SCENARIO("Initialize a session")
    {

        struct SpySessionDelegate: public session::SessionDelegate
        {
            int sessionWillInitializeCalled = 0;
            void sessionWillInitialize() {
                sessionWillInitializeCalled += 1;
            }
        };


        GIVEN("a session")
        {
            auto delegateSpy = std::make_shared<SpySessionDelegate>();
            auto session = session::Session(delegateSpy);

            
            auto status = session.state()->get<session::state::SessionStatus>(session::state::Key::Status);
            REQUIRE(status == session::state::SessionStatus::Inactive);

            WHEN("the session is initialized")
            {
                session.initialize();
                THEN("the session state should be set to 'initializing'")
                {
                    status = session.state()->get<session::state::SessionStatus>(session::state::Key::Status);
                    REQUIRE(status == session::state::SessionStatus::Initializing);
                }

                THEN("the delegate's session will initialized method is called")
                {
                    REQUIRE(delegateSpy ->sessionWillInitializeCalled == 1);
                }
            }

        }
    }
}
