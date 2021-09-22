// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* events.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>

namespace indiemotion::events
{

    enum class EventKind
    {
        DebugMessage,
        PromptMessage
    };

    struct Event
    {
        EventKind kind;
        std::string name;
        std::string session_id;
        std::string time;
        // TODO EventBody body;

        static Event new_event(EventKind kind, std::string name, std::string session_id, std::string time)
        {
            return Event{kind, name, session_id, time};
        }
    };
}
