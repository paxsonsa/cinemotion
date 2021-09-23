#pragma once

#include <indiemotion/_common.hpp>
#include <indiemotion/server/server.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/session/session_delegate.hpp>

namespace indiemotion::session {
    class Session
    {
    private:
        std::shared_ptr<server::Connection> _m_conn = nullptr;
        std::shared_ptr<SessionDelegate> _m_delegate = nullptr;

        void onMessage(messages::Message message)
        {
            return;
        }

    public:
        // Default Constructor
        Session(std::shared_ptr<server::Connection> conn): _m_conn(conn) {
            _m_conn->bindMessageReciever([this](messages::Message message) {
                onMessage(std::move(message));
            });
        };

        // Default Constructor
        Session(std::shared_ptr<server::Connection> conn, std::shared_ptr<SessionDelegate> delegate): _m_conn(conn), _m_delegate(delegate) {
            _m_conn->bindMessageReciever([this](messages::Message message) {
                onMessage(std::move(message));
            });
        };

        // Copy the resource (copy constructor)
        // We do not allow for the Sesion Object ot be copied
        Session(const Session &rhs) = delete;

        // Transfer Ownership (move constructor)
        Session(Session &&rhs) noexcept 
        {
            _m_delegate = std::exchange(rhs._m_delegate, nullptr);
            _m_conn = std::exchange(rhs._m_conn, nullptr);
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
            swap(_m_conn, rhs._m_conn);
            swap(_m_delegate, rhs._m_delegate);
        }

        void set_delegate(std::shared_ptr<SessionDelegate> delegate)
        {
            _m_delegate = delegate;
        }
    };

}