// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp 

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/errors.hpp>
#include <indiemotion/messages/messages.hpp>
#include <indiemotion/responses/responses.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

SCENARIO("Initializing the Session")
{
    GIVEN("a new manager object")
    {
        auto manager = indiemotion::session::SessionManager();

        WHEN("manager.initalize() is called")
        {
            auto msg = manager.initialize();

            THEN("initialize() should have returned a response")
            {
                REQUIRE(msg);

                AND_THEN("the message should be a properly init message")
                {
                    REQUIRE(msg->kind() == responses::Kind::InitSession);
                    REQUIRE(msg->needsAcknowledgment() == true);
                }
            }
        }
    }

    GIVEN("an initialized session manager")
    {
        auto manager = session::SessionManager();
        auto msg = manager.initialize();
        auto id = msg->id();
        WHEN("the manager processes an ACK for the init message")
        {
            auto ackMsg = std::make_unique<messages::acknowledge::Message>(id);
            auto noMsg = manager.processMessage(std::move(ackMsg));

            THEN("no message should be returned")
            {
                REQUIRE_FALSE(noMsg.has_value());
            }

            THEN("the session should be in an activated status")
            {
                REQUIRE(manager.session()->status() == session::state::SessionStatus::Active);
            }
        }
    }
}

SCENARIO("Request for camera list fails")
{
    GIVEN("An uninitialized manager")
    {
        auto manager = session::SessionManager();

        WHEN("the client requests a camera list")
        {
            auto m_ptr = std::make_unique<messages::listCameras::Message>();
            THEN("an error response should be generated")
            {
                REQUIRE_THROWS_AS(manager.processMessage(std::move(m_ptr)), indiemotion::errors::SessionError);
            }
        }
    }
}

SCENARIO("Return a camera list")
{
    GIVEN("An active session,")
    {
        class DummyDelegate : public session::SessionDelegate
        {
        public:
            std::vector<std::string> cameras()
            {
                return std::vector<std::string>{"cam1", "cam2"};
                ;
            }
        };
        auto delegate = std::make_shared<DummyDelegate>();
        auto manager = session::SessionManager();
        manager.session()->bindSessionDelegate(delegate);
        manager.session()->activate();

        WHEN("the client requests a camera list,")
        {
            auto m_ptr = std::make_unique<messages::listCameras::Message>();
            auto resp = manager.processMessage(std::move(m_ptr));

            THEN("a camera list response should be generated.")
            {
                REQUIRE(resp.has_value());
                REQUIRE(resp.value()->kind() == responses::Kind::CameraList);
                std::unique_ptr<responses::cameraList::Response> ptr(
                    dynamic_cast<responses::cameraList::Response *>(resp->release()));
                REQUIRE(ptr->cameraNames() == delegate->cameras());
            }
        }
    }
}