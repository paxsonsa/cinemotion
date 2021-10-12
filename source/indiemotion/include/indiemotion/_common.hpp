// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* _common.hpp 

Header for providing common include across the library
*/
#pragma once
#include <any>
#include <exception>
#include <functional>
#include <iostream>
#include <memory>
#include <optional>
#include <sstream>
#include <string>
#include <type_traits>

#include <boost/format.hpp>
#include <fmt/core.h>
#include <spdlog/spdlog.h>

#include <indiemotion/common/boost.hpp>
#include <indiemotion/common/log.hpp>

#include <indiemotion-protobufs/messages.pb.h>

template <typename TO, typename FROM>
std::unique_ptr<TO> static_unique_pointer_cast(std::unique_ptr<FROM> &&old)
{
    return std::unique_ptr<TO>{static_cast<TO *>(old.release())};
    //conversion: unique_ptr<FROM>->FROM*->TO*->unique_ptr<TO>
}

template <typename E>
constexpr auto to_underlying(E e) noexcept
{
    return static_cast<std::underlying_type_t<E>>(e);
}