// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* Session.hpp 


*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/motion/session_delegate.hpp>

namespace indiemotion::motion
{
    class Session
    {
    private:
    public:
        // Default Constructor
        Session(){};

        // Copy the resource (copy constructor)
        Session(const Session &rhs) {}

        // Transfer Ownership (move constructor)
        Session(Session &&rhs) noexcept
        {
            // member = std::exchange(rhs.member, replacevalue);
        }

        // Make type `std::swap`able
        friend void swap(Session &a, Session &b) noexcept
        {
            a.swap(b);
        }

        // Destructor
        ~Session()
        {
            // std::cout << "Destroyed" << std::endl;
        }

        // Assignment by Value
        Session &operator=(Session copy)
        {
            copy.swap(*this);
            return *this;
        }

        void swap(Session &rhs) noexcept
        {
            // using std::swap;
            //swap(member, rhs.member);
        }

        void set_delegate(MotionSessionDelegatePtr delegate) {}
    };
}
