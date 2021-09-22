// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* Session.hpp 


*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/events.hpp>

#include <indiemotion/motion/session_delegate.hpp>

namespace indiemotion::motion
{
    class Session
    {
    private:
        std::shared_ptr<SessionDelegate> _m_delegate = nullptr;

    public:
        // Default Constructor
        Session(){};

        // Copy the resource (copy constructor)
        Session(const Session &rhs)
        {
            _m_delegate = rhs._m_delegate;
        }

        // Transfer Ownership (move constructor)
        Session(Session &&rhs) noexcept
        {
            _m_delegate = std::exchange(rhs._m_delegate, nullptr);
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
            using std::swap;
            swap(_m_delegate, rhs._m_delegate);
        }

        void set_delegate(std::shared_ptr<SessionDelegate> delegate)
        {
            _m_delegate = delegate;
        }

        void initialize()
        {
            _m_delegate->will_initialize_session();
            // Delegate willInitializeSession
            // TODO Send Initialization Command

            // TODO only call did initialize when client responds with init
            _m_delegate->did_initialize_session();
        }

        void shutdown()
        {
            _m_delegate->will_shutdown_session();
            // TODO Send Shutdown Command

            // TODO only call did* when client responds with shutdown ACK
            _m_delegate->did_shutdown_session();
        }

        void processMessage(indiemotion::events::Event event)
        {
        }
    };
}
