// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp 

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/motion.hpp>

TEST_CASE("Test Session Initialize")
{
    class SpyDelegate : public indiemotion::motion::SessionDelegate
    {
    private:
    public:
        bool willInitializeSessionCalled = false;

        SpyDelegate() {}
    };
    auto session = std::make_unique<Session>();
    auto spy = std::make_shared<SpyDelegate>();

    session->set_delegate(spy);
    session->initialize();

    CHECK_MESSAGE(spy->willInitializeSessionCalled, "Expected delegat 'will init' method to be called.");

#TODO Test Session willInitialize call
#TODO Test Sessio didInitialize through process messge
}
