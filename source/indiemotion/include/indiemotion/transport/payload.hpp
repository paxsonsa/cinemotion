// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* payload.hpp */
#pragma once
namespace indiemotion::transport
{
    template <typename Kind_T>
    class Payload
    {
    public:
        Payload() = default;
        virtual ~Payload() {}

        virtual Kind_T kind() = 0;
    };
}