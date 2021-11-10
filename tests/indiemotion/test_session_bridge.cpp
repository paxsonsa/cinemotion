// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/net/acknowledge.hpp>
#include <indiemotion/net/dispatcher.hpp>
#include <indiemotion/net/camera.hpp>
#include <indiemotion/net/error.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/motion.hpp>
#include <indiemotion/net/session.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

struct DummyDispatcher: NetMessageDispatcher {
    std::vector<std::unique_ptr<NetMessage>> messages {};

    void dispatch(std::unique_ptr <NetMessage> message)
    {
        messages.push_back(std::move(message));
    }
};

SCENARIO("Starting the Session")
{
    GIVEN("a new controller object")
    {
        auto session = std::make_shared<SessionController>();
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);

        WHEN("manager.start() is called")
        {
            bridge.start();
            auto response = std::move(dispatcher->messages[0]);

            THEN("start() should have returned a response")
            {
                REQUIRE(dispatcher->messages.size() == 1);
            }
            AND_THEN("the response should be a session start message")
            {
                REQUIRE(response->payloadType() == indiemotion::NetPayloadType::SessionStart);
            }
            AND_THEN("the response should be the session server info we expect")
            {
                auto payload = response->payloadPtrAs<NetSessionStart>();
                REQUIRE(payload->serverInfo.apiVersion == SessionBridge::APIVersion);
                REQUIRE(payload->serverInfo.features == 0);
                REQUIRE_FALSE(response->doesRequireAcknowledgement());
            }
            AND_THEN("session controller status should be starting")
            {
                REQUIRE(session->status() == SessionStatus::Starting);
            }
        }
    }

    GIVEN("an started session controller")
    {
        auto session = std::make_shared<SessionController>();
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);
        bridge.start();

        // Clear the messages so fair
        dispatcher->messages.clear();

        WHEN("the client sends a session activate message")
        {
            auto properties = SessionProperties();
            auto payloadPtr = std::make_unique<indiemotion::NetSessionActivate>(properties);
            auto message = indiemotion::netMakeMessage(std::move(payloadPtr));

            bridge.processMessage(std::move(message));

            THEN("no message should be returned")
            {
                REQUIRE_FALSE(dispatcher->messages.size() == 1);
            }

            AND_THEN("the session should be active")
            {
                REQUIRE(session->status() == SessionStatus::Activated);
            }
        }
    }
}

SCENARIO("acknowledge message with no ID should return an error")
{
    GIVEN("an active session bridge")
    {
        auto session = std::make_shared<SessionController>();
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);
        session->setStatus(SessionStatus::Activated);

        WHEN("an acknowledge message is processed without an inResponseToId")
        {
            auto ackPtr = std::make_unique<indiemotion::net::Acknowledge>();
            auto message = indiemotion::netMakeMessage(std::move(ackPtr));
            bridge.processMessage(std::move(message));

            REQUIRE(dispatcher->messages.size() == 1);
            auto expected = std::move(dispatcher->messages[0]);

            THEN("an invalid message error should be returned")
            {
                REQUIRE(expected->payloadType() == indiemotion::NetPayloadType::Error);
                auto payload = expected->payloadPtrAs<indiemotion::net::Error>();
                REQUIRE(payload->errorType == indiemotion::net::Error::Type::InvalidMessage);
            }
        }
    }
}

SCENARIO("List the Cameras")
{
    struct DummyDelegate : SessionControllerDelegate
    {

        std::vector<cameras::Camera> cameraList{
            cameras::Camera("cam1"),
            cameras::Camera("cam2"),
            cameras::Camera("cam3"),
        };

        std::vector<cameras::Camera> getAvailableCameras() override
        {
            return cameraList;
        }
    };

    GIVEN("a session bridge")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        session->setStatus(SessionStatus::Activated);

        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);


        WHEN("bridge processes list camera messages")
        {
            auto msg = std::make_unique<indiemotion::NetGetCameraList>();
            auto message = indiemotion::netMakeMessage(std::move(msg));
            bridge.processMessage(std::move(message));

            REQUIRE(dispatcher->messages.size() == 1);
            auto expected = std::move(dispatcher->messages[0]);

            THEN("the delegates camera list should be returned")
            {
                auto camList = expected->payloadPtrAs<indiemotion::NetCameraList>();
                REQUIRE(camList->cameras == delegate->cameraList);
            }
        }
    }
}

SCENARIO("Set the Camera Successfully")
{
    struct DummyDelegate : SessionControllerDelegate
    {

        std::vector<cameras::Camera> cameraList{
            cameras::Camera("cam1"),
            cameras::Camera("cam2"),
            cameras::Camera("cam3"),
        };

        std::optional<cameras::Camera> camera;

        std::vector<cameras::Camera> getAvailableCameras() override
        {
            return cameraList;
        }

        std::optional<cameras::Camera> getCameraById(std::string id) override
        {
            assert(id == "cam2" && "should not be possible in this test case.");
            return cameraList[1];
        }

        void didSetActiveCamera(cameras::Camera cam) override
        {
            camera = cam;
        }
    };

    GIVEN("a session bridge")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        session->setStatus(SessionStatus::Activated);

        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);

        WHEN("bridge processes setcamera messages")
        {
            auto payload = std::make_unique<indiemotion::NetSetActiveCamera>("cam2");
            auto message = indiemotion::netMakeMessage(std::move(payload));
            bridge.processMessage(std::move(message));


            REQUIRE(dispatcher->messages.size() == 1);
            auto response = std::move(dispatcher->messages[0]);

            THEN("the delegates camera should be set")
            {
                auto info = response->payloadPtrAs<indiemotion::NetActiveCameraInfo>();
                REQUIRE(info->camera->name == "cam2");
                REQUIRE(info->camera == delegate->camera);
                REQUIRE(info->camera->name == session->getActiveCamera().value().name);
            }
        }
    }
}

SCENARIO("updating the motion mode")
{
    struct DummyDelegate : SessionControllerDelegate
    {
        bool wasMotionModeDidUpdateCalled = false;
        MotionMode mode = MotionMode::Off;

        void didMotionSetMode(MotionMode m) override
        {
            wasMotionModeDidUpdateCalled = true;
            mode = m;
        }
    };

    GIVEN("a session bridge")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);

        session->setStatus(SessionStatus::Activated);

        WHEN("bridge processes setmotionmode=live message")
        {
            auto payload = std::make_unique<indiemotion::net::MotionSetMode>(MotionMode::Live);
            auto message = indiemotion::netMakeMessage(std::move(payload));

            bridge.processMessage(std::move(message));

            REQUIRE_FALSE(dispatcher->messages.size() > 0);

            THEN("the motion mode should be updated")
            {
                REQUIRE(session->currentMotionMode() == MotionMode::Live);
            }

            THEN("the delegates motion mode did update")
            {
                REQUIRE(delegate->wasMotionModeDidUpdateCalled);
                REQUIRE(delegate->mode == MotionMode::Live);
            }
        }

        WHEN("bridge processes setmotionmode=record message")
        {
            auto payload = std::make_unique<indiemotion::net::MotionSetMode>(MotionMode::Recording);
            auto message = indiemotion::netMakeMessage(std::move(payload));

            bridge.processMessage(std::move(message));
            REQUIRE_FALSE(dispatcher->messages.size() > 0);

            THEN("the motion mode should be updated")
            {
                REQUIRE(session->currentMotionMode() == MotionMode::Recording);
            }

            THEN("the delegates motion mode did update")
            {
                REQUIRE(delegate->wasMotionModeDidUpdateCalled);
                REQUIRE(delegate->mode == MotionMode::Recording);
            }
        }

        WHEN("bridge processes setmotionmode=off message")
        {
            auto payload = std::make_unique<indiemotion::net::MotionSetMode>(MotionMode::Off);
            auto message = indiemotion::netMakeMessage(std::move(payload));
            bridge.processMessage(std::move(message));

            REQUIRE_FALSE(dispatcher->messages.size() > 0);

            THEN("the motion mode should be updated")
            {
                REQUIRE(session->currentMotionMode() == MotionMode::Off);
            }

            THEN("the delegates motion mode did update")
            {
                REQUIRE(delegate->wasMotionModeDidUpdateCalled);
                REQUIRE(delegate->mode == MotionMode::Off);
            }
        }
    }
}

SCENARIO("updating the motion xform")
{
    struct DummyDelegate : SessionControllerDelegate
    {
        bool wasReceivedMotionUpdateCalled = false;
        MotionXForm xform;

        void receivedMotionUpdate(MotionXForm m) override
        {
            wasReceivedMotionUpdateCalled = true;
            xform = m;
        }
    };

    GIVEN("a fresh active session")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        session->setStatus(SessionStatus::Activated);
        session->setMotionMode(MotionMode::Live);

        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);

        WHEN("a motion message is processed")
        {
            auto xform = MotionXForm::create(
                1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
            auto payload = std::make_unique<indiemotion::net::MotionUpdateXForm>(xform);
            auto message = indiemotion::netMakeMessage(std::move(payload));
            bridge.processMessage(std::move(message));

            REQUIRE_FALSE(dispatcher->messages.size() > 0);

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
    struct DummyDelegate : SessionControllerDelegate
    {
        bool wasReceivedMotionUpdateCalled = false;
        MotionXForm xform;

        void receivedMotionUpdate(MotionXForm m) override
        {
            wasReceivedMotionUpdateCalled = true;
            xform = m;
        }
    };

    GIVEN("a fresh active session")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);

        session->setStatus(SessionStatus::Activated);

        WHEN("the session's motion mode is off and motion update is processed")
        {
            session->setMotionMode(MotionMode::Off);
            auto xform = MotionXForm::create(
                1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
            auto payload = std::make_unique<indiemotion::net::MotionUpdateXForm>(xform);
            auto message = indiemotion::netMakeMessage(std::move(payload));

            bridge.processMessage(std::move(message));


            THEN("delegate's recieved motion routine should NOT be invoked")
            {
                REQUIRE_FALSE(dispatcher->messages.size() > 0);
                REQUIRE_FALSE(delegate->wasReceivedMotionUpdateCalled);
                REQUIRE_FALSE(delegate->xform == xform);
            }
        }

        WHEN("the session's motion mode is live and motion update is processed")
        {
            session->setMotionMode(MotionMode::Live);
            auto xform = MotionXForm::create(
                1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
            auto payload = std::make_unique<indiemotion::net::MotionUpdateXForm>(xform);
            auto message = indiemotion::netMakeMessage(std::move(payload));
            bridge.processMessage(std::move(message));


            THEN("delegate's recieved motion routine should be invoked")
            {
                REQUIRE_FALSE(dispatcher->messages.size() > 0);
                REQUIRE(delegate->wasReceivedMotionUpdateCalled);
                REQUIRE(delegate->xform == xform);
            }
        }

        // Reset Sessions
        delegate->wasReceivedMotionUpdateCalled = false;

        WHEN("the session's motion mode is live and motion update is processed")
        {
            session->setMotionMode(MotionMode::Live);
            auto xform = MotionXForm::create(
                6.0f, 7.0f, 8.0f, 9.0f, 10.0f, 11.0f);
            auto payload = std::make_unique<indiemotion::net::MotionUpdateXForm>(xform);
            auto message = indiemotion::netMakeMessage(std::move(payload));
            bridge.processMessage(std::move(message));

            THEN("no response should be returned")
            {
                REQUIRE_FALSE(dispatcher->messages.size() > 0);
            }

            THEN("delegate's recieved motion routine should be invoked")
            {
                REQUIRE(delegate->wasReceivedMotionUpdateCalled);
                REQUIRE(delegate->xform == xform);
            }
        }
    }
}
