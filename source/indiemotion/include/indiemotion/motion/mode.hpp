// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* motion_state.hpp */
#pragma once

#include <indiemotion/common.hpp>

namespace indiemotion::motion
{
    /**
     * @brief A simple value for comparing mode values
     * 
     */
    enum class MotionMode
    {
        Off,
        Live,
        Recording
    };
}