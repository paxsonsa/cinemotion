// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp 

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/session/session.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/messages/acknowledge.hpp>
#include <indiemotion/messages/factory.hpp>

using namespace indiemotion;

TEST_CASE("Test Example")
{   
    auto session = std::make_shared<session::Session>();
    auto id = messages::message::ID(0);
    auto message = std::make_unique<messages::acknowledge::AcknowledgeMessage>(id);
    auto handler = messages::handler::factory::create("Acknowledge");
    handler->handleMessage(session, std::move(message));

}

TEST_CASE("Test Another Example")
{
    CHECK_EQ(1, 1);
}
