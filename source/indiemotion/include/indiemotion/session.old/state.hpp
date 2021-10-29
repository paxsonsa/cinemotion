// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* state.hpp 

*/
#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion::session::state
{

    enum class Status
    {
        Dead = -1,
        Inactive = 0,
        Error = 1,
        Initializing = 2,
        Active = 3
    };

    enum class Key
    {
        Status,
        Properties,
        ModeContext,
    };

    std::string keyname(Key key)
    {
        switch (key)
        {
        case Key::Status:
            return "status";
        case Key::Properties:
            return "properties";
        case Key::ModeContext:
            return "mode_context";
        };
    }

    class State
    {
    private:
        std::map<Key, std::any> _m_state_table{};

    public:
        State() = default;

        void set(Key key, std::any value)
        {
            _m_state_table[key] = value;
        }

        std::any getAny(Key key)
        {
            return _m_state_table[key];
        }

        template <typename T>
        T get(Key key)
        {
            return std::any_cast<T>(_m_state_table[key]);
        }
    };

}