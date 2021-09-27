#pragma once

#include <indiemotion/_common.hpp>
#include <indiemotion/server/server.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/session/session_delegate.hpp>

namespace indiemotion::session {

    struct Session {
        virtual void set_delegate(std::shared_ptr<SessionDelegate> delegate) = 0;
        virtual void initialize(properties::ClientProperties props) = 0;
    };

    class SessionImpl
    {
    
    private:
        std::shared_ptr<server::Connection> _m_conn = nullptr;
        std::shared_ptr<SessionDelegate> _m_delegate = nullptr;

    public:
        // Default Constructor
        SessionImpl(std::shared_ptr<server::Connection> conn): _m_conn(conn) {
            _m_conn->bindMessageReciever([this](messages::Message message) {});
        };

        // Default Constructor
        SessionImpl(std::shared_ptr<server::Connection> conn, std::shared_ptr<SessionDelegate> delegate): _m_conn(conn), _m_delegate(delegate) {
            _m_conn->bindMessageReciever([this](messages::Message message) {});
        };

        // Copy the resource (copy constructor)
        // We do not allow for the Sesion Object ot be copied
        SessionImpl(const SessionImpl &rhs) = delete;

        // Transfer Ownership (move constructor)
        SessionImpl(SessionImpl &&rhs) noexcept 
        {
            _m_delegate = std::exchange(rhs._m_delegate, nullptr);
            _m_conn = std::exchange(rhs._m_conn, nullptr);
        }

        // Make type `std::swap`able
        friend void swap(SessionImpl &a, SessionImpl &b) noexcept
        {
            a.swap(b);
        }

        // Destructor
        ~SessionImpl()
        {
            // std::cout << "Destroyed" << std::endl;
        }

        // Assignment by Value
        SessionImpl &operator=(SessionImpl copy)
        {
            copy.swap(*this);
            return *this;
        }

        void swap(SessionImpl &rhs) noexcept
        {
            using std::swap;
            swap(_m_conn, rhs._m_conn);
            swap(_m_delegate, rhs._m_delegate);
        }

        void set_delegate(std::shared_ptr<SessionDelegate> delegate)
        {
            _m_delegate = delegate;
        }

        /**
         * @brief Initialize the session for motion capture
         * 
         */
        void initialize() 
        {   
            // device::DeviceProperties info;
            // auto newInfo = _m_delegate->deviceInfo(info);
            
            // auto msg = messages::InitSessionMsg{
            //     newInfo
            // };

            // _m_conn->send(msg);

        }
    };

}