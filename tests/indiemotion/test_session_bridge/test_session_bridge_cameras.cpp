// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/net/message.hpp>
#include "indiemotion/cameras/camera.hpp"

using namespace indiemotion;

struct DummyDispatcher : NetMessageDispatcher {
    std::vector<NetMessage> messages{};

    void dispatch(NetMessage &&message) {
        messages.push_back(std::move(message));
    }
};

struct DummyDelegate : SessionControllerDelegate{
    bool sessionWillShutdownCalled = false;

    void will_shutdown_session() //override
    {
        sessionWillShutdownCalled = true;
    }
};

SCENARIO("Listing the Cameras")
{
    struct DummyDelegate : SessionControllerDelegate
    {
        std::vector<Camera> cameraList{
            Camera("cam1"),
            Camera("cam2"),
            Camera("cam3"),
        };

        std::vector<Camera> get_available_cameras() override
        {
            return cameraList;
        }
    };

    GIVEN("a session bridge")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);
        session->initialize();


        WHEN("bridge processes list camera messages")
        {
            auto message = netMakeMessage();
            message.mutable_get_camera_list();

            bridge.process_message(std::move(message));

            REQUIRE(dispatcher->messages.size() == 1);
            auto expected = std::move(dispatcher->messages[0]);

            THEN("the delegates camera list should be returned")
            {
                auto camList = expected.camera_list();
                REQUIRE(camList.cameras_size() == delegate->cameraList.size());
                REQUIRE(camList.cameras()[0].id() == "cam1");
                REQUIRE(camList.cameras()[1].id() == "cam2");
                REQUIRE(camList.cameras()[2].id() == "cam3");
            }
        }
    }
}


SCENARIO("Set the Camera Successfully")
{
    struct DummyDelegate : SessionControllerDelegate
    {

        std::vector<Camera> cameraList{
            Camera("cam1"),
            Camera("cam2"),
            Camera("cam3"),
        };

        std::optional<Camera> camera;

        std::vector<Camera> get_available_cameras() override
        {
            return cameraList;
        }

        std::optional<Camera> get_camera_by_name(std::string id) override
        {
            assert(id == "cam2" && "should not be possible in this test case.");
            return cameraList[1];
        }

        void did_set_active_camera(Camera cam) override
        {
            camera = cam;
        }
    };

    GIVEN("a session bridge")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        session->initialize();

        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);

        WHEN("bridge processes set camera messages")
        {
            auto message = netMakeMessage();
            auto payload = message.mutable_set_active_camera();
            payload->set_camera_id("cam2");
            bridge.process_message(std::move(message));

            THEN("the delegates camera should be set")
            {
                REQUIRE(dispatcher->messages.size() == 0);
                REQUIRE(delegate->camera.value().name == "cam2");
            }
        }
    }
}
