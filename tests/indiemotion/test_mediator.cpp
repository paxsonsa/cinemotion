// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* test_mediator.hpp 

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/_common.hpp>
#include <indiemotion/device.hpp>
#include <indiemotion/messages.hpp>
#include <indiemotion/server.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/properties.hpp>

using namespace indiemotion;

TEST_SUITE("test mediator/session integration")
{
    class DummyConnection : public server::Connection
    {
    public:
        messages::Message message;
        int sendCalled = 0;

        void bindMessageReciever(messages::MessageHandler handler) noexcept {}
        void send(messages::Message message)
        {
            sendCalled += 1;
        }
    };

    class DummySession : public session::Session
    {
    public:
        int initializeCalled = 0;
        properties::ClientProperties givenProps = properties::ClientProperties();

        void set_delegate(std::shared_ptr<session::SessionDelegate> delegate){};

        void initialize(properties::ClientProperties props)
        {
            initializeCalled += 1;
            givenProps = props;
        }
    };

    TEST_CASE("test initialized()")
    {
        auto session = std::make_shared<DummySession>();
        auto mediator = session::SessionMediator(session);
        auto msg = messages::InitSessionMsg(
            properties::ClientProperties(
                "someName",
                "iphone 13 pro",
                std::vector<std::string>{"1.0"}));

        mediator.handleMessage(msg);

        CHECK_MESSAGE(session->initializeCalled == 1, "initialized() should be called by mediator for a client init message.");
        CHECK_EQ(session->givenProps.name, "someName");
        CHECK_EQ(session->givenProps.deviceID, "iphone 13 pro");
        CHECK_EQ(session->givenProps.supportedAPIVersions[0], "1.0");
    }

    TEST_CASE("test ackInitialization() calls conn.send()")
    {
        auto conn = std::make_shared<DummyConnection>();
        auto mediator = session::ConnectionMediator(conn);

        session::FeatureSet features = session::FeatureSet(session::Features::VideoStreaming);

        auto props = properties::SessionProperties(
            "mysession",
            "1.0",
            features
        );

        auto uid = mediator.ackInitialization(props);

        CHECK_EQ(conn->sendCalled, 1);
        CHECK_EQ(conn->message.uid, uid);
    }
}

