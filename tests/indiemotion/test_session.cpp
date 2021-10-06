// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* test_session.cpp 

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/_common.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

TEST_SUITE("test session")
{
    SCENARIO("Initialize a session")
    {
        struct SpySessionDelegate: public session::SessionDelegate
        {
            
            std::string m_name = "testname";
            session::FeatureSet m_features = session::FeatureSet(session::Features::VideoStreaming);

            int sessionWillInitializeCalled = 0;
            void sessionWillInitialize() override
            {
                sessionWillInitializeCalled += 1;
            }

            int sessionDidInitializeCalled = 0;
            void sessionDidInitialize() override
            {
                sessionDidInitializeCalled += 1;
            }

            std::optional<std::string> name() override
            {
                return m_name;
            }

            std::optional<session::FeatureSet> supportedFeatures() override
            {
                return m_features;
            }
        };

        auto delegateSpy = std::make_shared<SpySessionDelegate>();
        auto session = session::Session(delegateSpy);

        GIVEN("a session with a provided delegate")
        {
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

                THEN("the session properties should be configured in the state using the delegates input.")
                {
                    auto props = session.state()->get<session::Properties>(session::state::Key::Properties);
                    REQUIRE(delegateSpy->m_name == props.name);
                    REQUIRE(delegateSpy->m_features == props.features);
                }

                THEN("the session properties should be updated")
                {
                    auto props = session.properties();
                    REQUIRE(delegateSpy->m_name == props.name);
                    REQUIRE(delegateSpy->m_features == props.features);
                }
            }

            WHEN("the session is activated")
            {
                session.activate();

                THEN("the delegate's did initialize should be called")
                {
                    REQUIRE(delegateSpy->sessionDidInitializeCalled == 1);
                }

                AND_THEN("the sessions status should be active")
                {
                    auto status = session.state()->get<session::state::SessionStatus>(session::state::Key::Status);
                    REQUIRE(status == session::state::SessionStatus::Active);
                }
            }
        }
    }
}
