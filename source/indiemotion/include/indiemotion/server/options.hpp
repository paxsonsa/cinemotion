// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* options.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>

namespace indiemotion::server
{
    struct Options
    {
        std::optional<std::string> address = {};
        std::optional<uint> port = {};
    };
}
