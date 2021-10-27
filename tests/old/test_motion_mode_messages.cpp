// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/errors.hpp>
#include <indiemotion/messages.hpp>
#include <indiemotion/motion.hpp>
#include <indiemotion/responses.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

SCENARIO("Initial Motion Mode should be Off")
{
    class DummyDelegate : public session::SessionDelegate
    {
    };

    GIVEN("a new active session")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto manager = session::SessionManager();
        manager.session()->bindSessionDelegate(delegate);
        manager.session()->activate();

        WHEN("the client sends a get mode message")
        {
            auto payload = std::make_unique<messages::motion::getmode::Payload>();
            auto message = messages::base::createMessage(std::move(payload));
            auto opt_response = manager.processMessage(std::move(message));

            THEN("the response should return the current motion mode on the session")
            {
                REQUIRE(opt_response);
                auto response = std::move(opt_response.value());
                REQUIRE(response->payloadKind() == responses::Kind::MotionCurrentMode);

                auto responsePayload = response->payloadPtrAs<responses::motion::curmode::Payload>();
                REQUIRE(responsePayload->mode() == manager.session()->motionMode());
            }
        }
    }
}

SCENARIO("Changing Motion Mode")
{
    class DummyDelegate : public session::SessionDelegate
    {
    public:
        int motionModeDidUpdateCalled = 0;
        std::optional<motion::ModeValue> newMode;

        void motionModeDidUpdate(motion::ModeValue mode)
        {
            motionModeDidUpdateCalled += 1;
            newMode = mode;
        }
    };

    GIVEN("a new active session")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto manager = session::SessionManager();
        manager.session()->bindSessionDelegate(delegate);
        manager.session()->activate();

        WHEN("the client sends a set mode message")
        {
            auto payload = std::make_unique<indiemotion::messages::motion::setmode::Payload>(
                indiemotion::motion::ModeValue::Live);
            auto message = messages::base::createMessage(std::move(payload));
            auto opt_response = manager.processMessage(std::move(message));

            THEN("the server should acknowledge the message")
            {
                REQUIRE(opt_response);
                auto response = std::move(opt_response.value());
                std::cout << "payLoad: " << responses::kindToStr(response->payloadKind()) << "\n";
                REQUIRE(response->payloadKind() == responses::Kind::Acknowledgment);
            }

            THEN("the current mode value should be equal to the requested value")
            {
                REQUIRE(manager.session()->motionMode() == motion::ModeValue::Live);
            }

            AND_THEN("the delegate should have been notified of the updated mode")
            {
                REQUIRE(delegate->motionModeDidUpdateCalled == 1);
                REQUIRE(delegate->newMode == motion::ModeValue::Live);
            }
        }
    }
}
