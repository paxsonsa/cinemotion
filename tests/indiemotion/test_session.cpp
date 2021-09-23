// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp 

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/_common.hpp>
#include <indiemotion/messages.hpp>
#include <indiemotion/server.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

TEST_CASE("Test Session Binds to Connection")
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

// TODO Test Session willInitialize call
// TODO Test Sessio didInitialize through process messge
}
