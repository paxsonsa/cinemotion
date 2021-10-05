// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* factory.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/messages/_factory.hpp>
#include <indiemotion/messages/acknowledge.hpp>


namespace indiemotion::messages::handler
{
    using factory = _factory<
        Handler, 
        messages::acknowledge::AckMessageHandler
    >;
} // namespace indiemotion::messages::handler
