#pragma once

#include <indiemotion/_common.hpp>
#include <indiemotion/version.hpp>
#include <indiemotion/server/server.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/session/state.hpp>
#include <indiemotion/session/properties.hpp>
#include <indiemotion/session/session_delegate.hpp>

namespace indiemotion::session
{

    class Session
    {

    private:
        std::shared_ptr<SessionDelegate> _m_delegate = nullptr;
        std::shared_ptr<state::State> _m_state = nullptr;

    public:
        // Default Constructor
        Session()
        {
            _initializeState();
        };

        Session(std::shared_ptr<SessionDelegate> delegate)
            : Session()
        {
            _m_delegate = delegate;
        }

        // Copy the resource (copy constructor)
        // We do not allow for the Sesion Object ot be copied
        Session(const Session &rhs) = delete;

        // Transfer Ownership (move constructor)
        Session(Session &&rhs) noexcept
        {
            _m_delegate = std::exchange(rhs._m_delegate, nullptr);
            _m_state = std::exchange(rhs._m_state, nullptr);
        }

        // Make type `std::swap`able
        friend void swap(Session &a, Session &b) noexcept
        {
            a.swap(b);
        }

        // Destructor
        ~Session() {}

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
            swap(_m_state, rhs._m_state);
        }

        std::shared_ptr<state::State> state()
        {
            return _m_state;
        }

        void bind_delegate(std::shared_ptr<SessionDelegate> delegate)
        {
            _m_delegate = delegate;
        }

        /**
         * @brief Initialize the session for motion capture
         * 
         */
        void initialize()
        {
            if (_m_delegate)
            {
                _m_delegate->sessionWillInitialize();
            }
            _m_state->set(session::state::Key::Status, session::state::SessionStatus::Initializing);
        }

        SessionProperties properties()
        {
            return SessionProperties{
                // TODO Make Constant
                "indiemotion-server",
                indiemotion::API_VERSION,
                FeatureSet(0)};
        }

    private:
        void _initializeState()
        {
            _m_state = std::make_shared<state::State>();
            _m_state->set(state::Key::Status, state::SessionStatus::Inactive);
        }
    };

}