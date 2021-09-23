// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* session_delegate.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/device/device.hpp>
namespace indiemotion::session
{
    class SessionDelegate {
        public:
            virtual device::DeviceInfo deviceInfo(device::DeviceInfo intialInfo) = 0;
    };
}
