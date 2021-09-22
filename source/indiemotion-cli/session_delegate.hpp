// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* session_delegate.hpp 
*/
#include <indiemotion/motion.hpp>

#include "repl.hpp"
class SessionDelegate : public indiemotion::motion::SessionDelegate
{
private:
    std::weak_ptr<ReplWriter> _m_writer;

public:
    // Default Constructor
    SessionDelegate(std::shared_ptr<ReplWriter> writer) : _m_writer(writer){};

    // Copy the resource (copy constructor)
    SessionDelegate(const SessionDelegate &rhs)
    {
        _m_writer = rhs._m_writer;
    }

    // Transfer Ownership (move constructor)
    SessionDelegate(SessionDelegate &&rhs) noexcept
    {
        _m_writer = rhs._m_writer;
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
        using std::swap;
        swap(_m_writer, rhs._m_writer);
    }
};
