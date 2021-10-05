// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* factory.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>

namespace indiemotion::messages::handler
{
    template<class Base, class... Ps>
    struct _factory
    {
        static std::unique_ptr<Base> make_handler(std::string_view kind, std::unique_ptr<Base> default_type = nullptr)
        {
            static_assert((std::is_base_of_v<Base, Ps> && ...), "");

            std::unique_ptr<Base> result = nullptr;
            int dummy[] = {
                ([&]() {
                    if (result == nullptr && kind == Ps::kind) {
                        result = std::make_unique<Ps>();
                    }
                }(), 0) ...
            };
            if (result == nullptr) {
                result = std::move(default_type);
            }
            return result;
        }
    };
} // namespace indiemotion::handler



