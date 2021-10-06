// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp 

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/_common.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/messages/acknowledge.hpp>
#include <indiemotion/messages/cameras.hpp>

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
                    REQUIRE(msg->kind() == messages::response::kind::InitSession);
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
            auto ackMsg = std::make_unique<messages::acknowledge::AcknowledgeMessage>(id);
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

SCENARIO("Client Requests Camera List")
{
    GIVEN("An uninitialized manager")
    {
        auto manager = session::SessionManager();

        WHEN("the client requests a camera list")
        {
            THEN("an error response should be generated")
            {}
        }
    }

    GIVEN("An active session,")
    {
        auto manager = session::SessionManager();
        auto names = std::vector<std::string>{"cam1", "cam2"};
        manager.session()->activate();

        WHEN("the client requests a camera list,")
        {
            auto m_ptr = std::make_unique<messages::cameras::ListCamerasMessage>();
            auto resp = manager.processMessage(std::move(m_ptr));

            THEN("a camera list response should be generated.")
            {   
                REQUIRE(resp.has_value());
                REQUIRE(resp.value()->kind() == messages::response::kind::ListCameras);
                std::unique_ptr<messages::cameras::CameraListResponse> ptr(
                    dynamic_cast<messages::cameras::CameraListResponse*>(resp->release())
                );
                REQUIRE(ptr->cameraNames() == names);
            }
        }
    }
}