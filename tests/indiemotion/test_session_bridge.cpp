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
#include <indiemotion/net/error.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/motion.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

SCENARIO("Initializing the Session")
{
    GIVEN("a new manager object")
    {
        auto name = "SessionName";
        auto session = std::make_shared<session::Session>();
        auto bridge = session::SessionBridge(name, session);

        WHEN("manager.initalize() is called")
        {
            auto response = bridge.initialize();

            THEN("initialize() should have returned a response")
            {
                REQUIRE(response);

                AND_THEN("the response should be a properly init message")
                {
                    REQUIRE(response->payloadType() == indiemotion::net::PayloadType::SessionInitilization);
                }

                AND_THEN("the repsonse should be the session properties we expect")
                {
                    auto properties = response->payloadPtrAs<session::SessionProperties>();
                    REQUIRE(properties->name == name);
                    REQUIRE(properties->apiVersion == session::SessionBridge::APIVersion);
                    REQUIRE(properties->features == 0);
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

SCENARIO("acknowledge message with no ID should return an error")
{
    GIVEN("an active session bridge")
    {
        auto session = std::make_shared<session::Session>();
        auto bridge = indiemotion::session::SessionBridge(session);
        session->setStatus(session::Status::Activated);

        WHEN("an acknowledge message is processed without an inResponseToId")
        {
            auto ackPtr = std::make_unique<indiemotion::net::Acknowledge>();
            auto message = indiemotion::net::createMessage(std::move(ackPtr));
            auto expected = bridge.processMessage(std::move(message));

            THEN("an invalid message error should be returned")
            {
                REQUIRE(expected);
                REQUIRE(expected.value()->payloadType() == indiemotion::net::PayloadType::Error);
                auto payload = expected.value()->payloadPtrAs<indiemotion::net::Error>();
                REQUIRE(payload->errorType == indiemotion::net::ErrorType::InvalidMessage);
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

        std::vector<cameras::Camera> getAvailableCameras()
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

SCENARIO("Set the Camera Successfully")
{
    struct DummyDelegate : session::Delegate
    {

        std::vector<cameras::Camera> cameraList{
            cameras::Camera("cam1"),
            cameras::Camera("cam2"),
            cameras::Camera("cam3"),
        };

        std::optional<cameras::Camera> camera;

        std::vector<cameras::Camera> getAvailableCameras()
        {
            return cameraList;
        }

        std::optional<cameras::Camera> getCameraById(std::string id)
        {
            assert(id == "cam2" && "should not be possible in this test case.");
            return cameraList[1];
        }

        void didSetActiveCamera(cameras::Camera cam)
        {
            camera = cam;
        }
    };

    GIVEN("a session bridge")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<session::Session>(delegate);
        session->setStatus(session::Status::Activated);
        auto bridge = indiemotion::session::SessionBridge(session);

        WHEN("bridge processes setcamera messages")
        {
            auto payload = std::make_unique<indiemotion::net::SetCamera>("cam2");
            auto message = indiemotion::net::createMessage(std::move(payload));
            auto response = bridge.processMessage(std::move(message));
            THEN("the delegates camera should be set")
            {
                REQUIRE(response);
                auto info = response.value()->payloadPtrAs<indiemotion::net::CameraInfo>();
                REQUIRE(info->camera->name == "cam2");
                REQUIRE(info->camera == delegate->camera);
                REQUIRE(info->camera->name == session->getActiveCamera().value().name);
            }
        }
    }
}

SCENARIO("updating the motion mode")
{
    struct DummyDelegate : session::Delegate
    {
        bool wasMotionModeDidUpdateCalled = false;
        motion::MotionMode mode = motion::MotionMode::Off;

        void didSetMotionMode(motion::MotionMode m)
        {
            wasMotionModeDidUpdateCalled = true;
            mode = m;
        }
    };

    GIVEN("a session bridge")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<session::Session>(delegate);
        session->setStatus(session::Status::Activated);
        auto bridge = indiemotion::session::SessionBridge(session);

        WHEN("bridge processes setmotionmode=live message")
        {
            auto payload = std::make_unique<indiemotion::net::SetMotionMode>(motion::MotionMode::Live);
            auto message = indiemotion::net::createMessage(std::move(payload));
            auto response = bridge.processMessage(std::move(message));
            REQUIRE_FALSE(response);

            THEN("the motion mode should be updated")
            {
                REQUIRE(session->currentMotionMode() == motion::MotionMode::Live);
            }

            THEN("the delegates motion mode did update")
            {
                REQUIRE(delegate->wasMotionModeDidUpdateCalled);
                REQUIRE(delegate->mode == motion::MotionMode::Live);
            }
        }

        WHEN("bridge processes setmotionmode=record message")
        {
            auto payload = std::make_unique<indiemotion::net::SetMotionMode>(motion::MotionMode::Recording);
            auto message = indiemotion::net::createMessage(std::move(payload));
            auto response = bridge.processMessage(std::move(message));
            REQUIRE(!response);

            THEN("the motion mode should be updated")
            {
                REQUIRE(session->currentMotionMode() == motion::MotionMode::Recording);
            }

            THEN("the delegates motion mode did update")
            {
                REQUIRE(delegate->wasMotionModeDidUpdateCalled);
                REQUIRE(delegate->mode == motion::MotionMode::Recording);
            }
        }

        WHEN("bridge processes setmotionmode=off message")
        {
            auto payload = std::make_unique<indiemotion::net::SetMotionMode>(motion::MotionMode::Off);
            auto message = indiemotion::net::createMessage(std::move(payload));
            auto response = bridge.processMessage(std::move(message));
            REQUIRE(!response);

            THEN("the motion mode should be updated")
            {
                REQUIRE(session->currentMotionMode() == motion::MotionMode::Off);
            }

            THEN("the delegates motion mode did update")
            {
                REQUIRE(delegate->wasMotionModeDidUpdateCalled);
                REQUIRE(delegate->mode == motion::MotionMode::Off);
            }
        }
    }
}

SCENARIO("updating the motion xform")
{
    struct DummyDelegate : session::Delegate
    {
        bool wasReceivedMotionUpdateCalled = false;
        motion::MotionXForm xform;

        void recievedMotionUpdate(motion::MotionXForm m)
        {
            wasReceivedMotionUpdateCalled = true;
            xform = m;
        }
    };

    GIVEN("a fresh active session")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<session::Session>(delegate);
        session->setStatus(session::Status::Activated);
        session->setMotionMode(motion::MotionMode::Live);
        auto bridge = indiemotion::session::SessionBridge(session);

        WHEN("a motion message is processed")
        {
            auto xform = motion::MotionXForm::create(
                1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
            auto payload = std::make_unique<indiemotion::net::UpdateMotionXForm>(xform);
            auto message = indiemotion::net::createMessage(std::move(payload));
            auto response = bridge.processMessage(std::move(message));

            THEN("no response should be returned")
            {
                REQUIRE_FALSE(response);
            }

            THEN("delegate's recieved motion routine should be invoked")
            {
                REQUIRE(delegate->wasReceivedMotionUpdateCalled);
                REQUIRE(delegate->xform == xform);
            }
        }
    }
}

SCENARIO("updating the motion xform when motion mode is not live or recording")
{
    struct DummyDelegate : session::Delegate
    {
        bool wasReceivedMotionUpdateCalled = false;
        motion::MotionXForm xform;

        void recievedMotionUpdate(motion::MotionXForm m)
        {
            wasReceivedMotionUpdateCalled = true;
            xform = m;
        }
    };

    GIVEN("a fresh active session")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<session::Session>(delegate);
        session->setStatus(session::Status::Activated);
        auto bridge = indiemotion::session::SessionBridge(session);

        WHEN("the session's motion mode is off and motion update is processed")
        {
            session->setMotionMode(motion::MotionMode::Off);
            auto xform = motion::MotionXForm::create(
                1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
            auto payload = std::make_unique<indiemotion::net::UpdateMotionXForm>(xform);
            auto message = indiemotion::net::createMessage(std::move(payload));
            auto response = bridge.processMessage(std::move(message));

            THEN("no response should be returned")
            {
                REQUIRE_FALSE(response);
            }

            THEN("delegate's recieved motion routine should NOT be invoked")
            {
                REQUIRE_FALSE(delegate->wasReceivedMotionUpdateCalled);
                REQUIRE_FALSE(delegate->xform == xform);
            }
        }

        WHEN("the session's motion mode is live and motion update is processed")
        {
            session->setMotionMode(motion::MotionMode::Live);
            auto xform = motion::MotionXForm::create(
                1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
            auto payload = std::make_unique<indiemotion::net::UpdateMotionXForm>(xform);
            auto message = indiemotion::net::createMessage(std::move(payload));
            auto response = bridge.processMessage(std::move(message));

            THEN("no response should be returned")
            {
                REQUIRE_FALSE(response);
            }

            THEN("delegate's recieved motion routine should be invoked")
            {
                REQUIRE(delegate->wasReceivedMotionUpdateCalled);
                REQUIRE(delegate->xform == xform);
            }
        }

        // Reset Sessions
        delegate->wasReceivedMotionUpdateCalled = false;

        WHEN("the session's motion mode is live and motion update is processed")
        {
            session->setMotionMode(motion::MotionMode::Live);
            auto xform = motion::MotionXForm::create(
                6.0f, 7.0f, 8.0f, 9.0f, 10.0f, 11.0f);
            auto payload = std::make_unique<indiemotion::net::UpdateMotionXForm>(xform);
            auto message = indiemotion::net::createMessage(std::move(payload));
            auto response = bridge.processMessage(std::move(message));

            THEN("no response should be returned")
            {
                REQUIRE_FALSE(response);
            }

            THEN("delegate's recieved motion routine should be invoked")
            {
                REQUIRE(delegate->wasReceivedMotionUpdateCalled);
                REQUIRE(delegate->xform == xform);
            }
        }
    }
}
