// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* test_list_cameras_messages.cpp

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
// #include <indiemotion/errors.hpp>
// #include <indiemotion/messages/base/wrapper.hpp>
// #include <indiemotion/messages/cameras/list/payload.hpp>
#include <indiemotion/messages/messages.hpp>
#include <indiemotion/responses/cameras/list/payload.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/transport/header.hpp>

#include <string>
#include <vector>

SCENARIO("Processing a ListCameras Message")
{
    class DummyDelegate : public indiemotion::session::SessionDelegate
    {
    public:
        std::vector<std::string> cameras()
        {
            return std::vector<std::string>{"cam1", "cam2"};
            ;
        }
    };
    GIVEN("An active session and with a bound delegate")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto manager = indiemotion::session::SessionManager();
        manager.session()->bindSessionDelegate(delegate);
        manager.session()->activate();

        WHEN("the we process a ListCameras Message")
        {
            auto header = std::make_unique<indiemotion::transport::Header>("messageId");
            auto payload = std::make_unique<indiemotion::messages::cameras::list::Payload>();
            auto container = std::make_unique<indiemotion::messages::base::Wrapper>(std::move(header),
                                                                                    std::move(payload));

            auto resp = manager.processMessage(std::move(container));
            THEN("a camera list response should exist.")
            {
                REQUIRE(resp.has_value());
                REQUIRE(resp.value()->payloadKind() == indiemotion::responses::Kind::CameraList);
            }
            AND_THEN("the camera list should match what the delegate returned")
            {
                auto ctn = std::move(resp.value());
                auto ptr = dynamic_cast<indiemotion::responses::cameras::list::Payload *>(ctn->payload().lock().get());
                REQUIRE(ptr->cameraNames() == delegate->cameras());
            }
        }
    }
}

SCENARIO("Request for camera list fails")
{
    GIVEN("An uninitialized manager")
    {
        auto manager = indiemotion::session::SessionManager();

        WHEN("the client requests a camera list")
        {
            auto header = std::make_unique<indiemotion::transport::Header>("messageId");
            auto payload = std::make_unique<indiemotion::messages::cameras::list::Payload>();
            auto container = std::make_unique<indiemotion::messages::base::Wrapper>(std::move(header),
                                                                                    std::move(payload));
            THEN("an error response should be generated")
            {
                REQUIRE_THROWS_AS(manager.processMessage(std::move(container)), indiemotion::errors::SessionError);
            }
        }
    }
}
