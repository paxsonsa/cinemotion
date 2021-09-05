// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* session_delegate.hpp 
*/
#include <indiemotion/motion.hpp>
class SessionDelegate : public indiemotion::motion::MotionSessionDelegate
{
private:
public:
    // Default Constructor
    SessionDelegate(){};

    // Copy the resource (copy constructor)
    SessionDelegate(const SessionDelegate &rhs) {}

    // Transfer Ownership (move constructor)
    SessionDelegate(SessionDelegate &&rhs) noexcept
    {
        // member = std::exchange(rhs.member, replacevalue);
    }

    // Make type `std::swap`able
    friend void swap(SessionDelegate &lhs, SessionDelegate &rhs) noexcept
    {
        lhs.swap(rhs);
    }

    // Destructor
    ~SessionDelegate() {}

    // Assignment by Value
    SessionDelegate &operator=(SessionDelegate copy)
    {
        copy.swap(*this);
        return *this;
    }

    void swap(SessionDelegate &rhs) noexcept
    {
        // using std::swap;
        //swap(member, rhs.member);
    }
};
