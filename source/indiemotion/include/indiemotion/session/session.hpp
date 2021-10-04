#pragma once

#include <indiemotion/_common.hpp>
#include <indiemotion/version.hpp>
#include <indiemotion/server/server.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/session/state.hpp>
#include <indiemotion/session/properties.hpp>
#include <indiemotion/session/delegate.hpp>

namespace indiemotion::session
{

    // Forward Declaration
    class SessionManager;

    class Session
    {

        friend class SessionManager;

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

        /**
         * @brief Bind the delegate to the session
         * 
         * @param delegate 
         */
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
            auto properties = Properties{
                // TODO Make Constant
                "indiemotion-server",
                indiemotion::API_VERSION,
                FeatureSet(0)};

            if (_m_delegate)
            {
                _m_delegate->sessionWillInitialize();

                if (auto name = _m_delegate->name())
                {
                    properties.name = *name;
                }

                if (auto features = _m_delegate->supportedFeatures())
                {
                    properties.features = *features;
                }
            }
            _m_state->set(session::state::Key::Properties, properties);
            _m_state->set(session::state::Key::Status, session::state::SessionStatus::Initializing);
        }

        /**
         * @brief Returns the current properties for the session.
         * 
         * @return Properties 
         */
        Properties properties()
        {
            return _m_state->get<Properties>(session::state::Key::Properties);
        }

        /**
         * @brief Returns a pointer to the session state.
         * 
         * @return std::shared_ptr<state::State> 
         */
        std::shared_ptr<state::State> state()
        {
            return _m_state;
        }

        void activate()
        {
            _m_state->set(state::Key::Status, state::SessionStatus::Active);
            if (_m_delegate)
            {
                _m_delegate->sessionDidInitialize();
            }
        }

        /**
         * @brief Current Status of the session
         * 
         * @return state::SessionStatus 
         */
        state::SessionStatus status()
        {
            return _m_state->get<state::SessionStatus>(state::Key::Status);
        }

    private:
        /**
         * @brief Initialize the state object on this class
         * 
         */
        void _initializeState()
        {
            _m_state = std::make_shared<state::State>();
            _m_state->set(state::Key::Status, state::SessionStatus::Inactive);
        }
    };

}