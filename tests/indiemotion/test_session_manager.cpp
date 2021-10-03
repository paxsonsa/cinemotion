// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.hpp 

*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/_common.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/messages.hpp>

using namespace indiemotion;

TEST_SUITE("test session manager")
{

    TEST_CASE("test session initialization")
    {
        auto manager = session::SessionManager();
        if (auto opt_msg = manager.initialize(); opt_msg)
        {
        }
        else
        {
            FAIL("expected intialize to return a valid message");
        }

        
    }

}