// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/errors.hpp>
#include <indiemotion/net/acknowledge.hpp>
#include <indiemotion/net/camera.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

SCENARIO("Initializing the Session")
{
    GIVEN("a new manager object")
    {
        auto session = std::make_shared<session::Session>();
        auto bridge = indiemotion::session::SessionBridge(session);

        WHEN("manager.initalize() is called")
        {
            auto response = bridge.initialize();

            THEN("initialize() should have returned a response")
            {
                REQUIRE(response);

                AND_THEN("the message should be a properly init message")
                {
                    REQUIRE(response->payloadType() == indiemotion::net::PayloadType::SessionInitilization);
                }
            }
        }
    }

    GIVEN("an initialized session manager")
    {
        auto session = std::make_shared<session::Session>();
        auto bridge = indiemotion::session::SessionBridge(session);
        auto response = bridge.initialize();

        REQUIRE(response->doesRequireAcknowledgement());
        REQUIRE(session->status() == session::Status::Initialized);

        WHEN("the client sends an acknowledge message")
        {
            auto ackPtr = std::make_unique<indiemotion::net::Acknowledge>();
            auto message = indiemotion::net::createMessage(response->id(), std::move(ackPtr));
            auto expected = bridge.processMessage(std::move(message));
            THEN("no message should be returned")
            {
                REQUIRE_FALSE(expected.has_value());
            }

            AND_THEN("the sesison should be active")
            {
                REQUIRE(session->status() == session::Status::Activated);
            }
        }
    }
}

SCENARIO("List the Cameras")
{
    struct DummyDelegate : session::Delegate
    {

        std::vector<cameras::Camera> cameraList{
            cameras::Camera("cam1"),
            cameras::Camera("cam2"),
            cameras::Camera("cam3"),
        };

        std::vector<cameras::Camera> cameras()
        {
            return cameraList;
        }
    };

    GIVEN("a session bridge")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<session::Session>(delegate);
        session->setStatus(session::Status::Activated);
        auto bridge = indiemotion::session::SessionBridge(session);

        WHEN("bridge processes list camera messages")
        {
            auto msg = std::make_unique<indiemotion::net::GetCameraList>();
            auto message = indiemotion::net::createMessage(std::move(msg));
            auto response = bridge.processMessage(std::move(message));
            THEN("the delegates camera list should be returned")
            {
                REQUIRE(response);
                auto camList = response.value()->payloadPtrAs<indiemotion::net::CameraList>();
                REQUIRE(camList->cameras == delegate->cameraList);
            }
        }
    }
}