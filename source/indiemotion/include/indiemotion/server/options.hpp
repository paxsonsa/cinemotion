// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* options.hpp 

*/
#pragma once
#include <boost/asio.hpp>

#include <indiemotion/common.hpp>

namespace indiemotion
{
    struct ServerOptions
    {
        std::optional<std::string> address = {};
        std::optional<unsigned short> port = {};
    };
}
