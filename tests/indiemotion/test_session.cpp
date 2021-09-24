// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp 

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/_common.hpp>
#include <indiemotion/device.hpp>
#include <indiemotion/messages.hpp>
#include <indiemotion/server.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

TEST_SUITE("session intialization")
{
    TEST_CASE("test session binds to connection")
    {
        class DummyConnection : public server::Connection
        {
        public:
            int bindMessageRecieverCallCount = 0;

            void bindMessageReciever(messages::MessageHandler handler) noexcept
            {
                bindMessageRecieverCallCount += 1;
            }
            void send(messages::Message messages) {}
        };

        auto conn = std::make_shared<DummyConnection>();
        auto session = std::make_unique<session::Session>(conn);

        CHECK_MESSAGE(conn->bindMessageRecieverCallCount == 1,
                      "expected bindMessageReciever to only be called "
                      "once when the session is instanced");
    }

    TEST_CASE("test session initialize()")
    {
        class DummyConnection : public server::Connection
        {
        public:
            int bindMessageRecieverCallCount = 0;
            std::vector<messages::Message> sents_messages{};

            void bindMessageReciever(messages::MessageHandler handler) noexcept
            {
                bindMessageRecieverCallCount += 1;
            }
            void send(messages::Message messages)
            {
                sents_messages.push_back(messages);
            }
        };

        class FakeDelegate : public session::SessionDelegate
        {
        public:
            bool deviceInfoCalled = false;

            device::DeviceProperties deviceInfo(device::DeviceProperties initial)
            {
                deviceInfoCalled = true;
                return initial;
            }
        };

        auto delegate = std::make_shared<FakeDelegate>();
        auto conn = std::make_shared<DummyConnection>();
        auto session = std::make_unique<session::Session>(conn, delegate);

        session->initialize();

        SUBCASE("expect deviceInfo to be called")
        {
            CHECK(delegate->deviceInfoCalled);
        }

        SUBCASE("expect conn to recieve new init message")
        {
            CHECK_MESSAGE(conn->sents_messages.size() == 1, "expected only one queue to be sent");
            CHECK_MESSAGE(conn->sents_messages[0].kind == messages::Message::Kind::InitSession, "message should a InitSession message.");
        }
    }

    TEST_CASE("test session process -> init client session")
    {
        class DummyConnection : public server::Connection
        {
        public:
            std::vector<messages::MessageHandler> handlers;
            std::vector<messages::Message> sents_messages{};

            void bindMessageReciever(const messages::MessageHandler handler) noexcept
            {
                handlers.push_back(handler);
            }
            void send(const messages::Message messages)
            {
                sents_messages.push_back(messages);
            }
        };

        class FakeDelegate : public session::SessionDelegate
        {
        public:
            bool deviceInfoCalled = false;

            device::DeviceProperties deviceInfo(const device::DeviceProperties given)
            {
                deviceInfoCalled = true;
                return given;
            }
        };

        auto delegate = std::make_shared<FakeDelegate>();
        auto conn = std::make_shared<DummyConnection>();
        auto session = std::make_unique<session::Session>(conn, delegate);

        auto handler = conn->handlers[0];
        messages::Message message = messages::ClientInitSessionMsg("my message");
        handler(message);

        CHECK_MESSAGE(delegate->deviceInfoCalled, "the delegate should be called when the session handles a message");
        CHECK_MESSAGE(conn->sents_messages.size() == 1, "the session should be sending an ACK message.");
    }
}