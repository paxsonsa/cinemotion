// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* session_delegate.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>

namespace indiemotion::motion
{
    class SessionDelegate
    {
    public:
        virtual void will_initialize_session(){};
        virtual void did_initialize_session(){};

        virtual void will_shutdown_session(){};
        virtual void did_shutdown_session(){};
    };
}
