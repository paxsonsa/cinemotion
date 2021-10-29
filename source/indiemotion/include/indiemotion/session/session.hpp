#pragma once
#include <indiemotion/session/delegate.hpp>

namespace indiemotion::session
{

    enum class SessionStatus
    {
        Offline,
        Initialized,
        Activated,
    };

    class Session
    {
    private:
        SessionStatus _m_status = SessionStatus::Offline;
        std::shared_ptr<SessionDelegate> _m_delegate = nullptr;

    public:
        Session() {}

        Session(std::shared_ptr<SessionDelegate> delegate) : Session()
        {
            _m_delegate = delegate;
        }

        SessionStatus status() const { return _m_status; }
        void setStatus(SessionStatus status) { _m_status = status; }
    };
}