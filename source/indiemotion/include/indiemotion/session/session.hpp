#pragma once
#include <indiemotion/session/delegate.hpp>

namespace indiemotion::session
{
    class Session
    {
    private:
        std::shared_ptr<SessionDelegate> _m_delegate = nullptr;

    public:
        Session() {}

        Session(std::shared_ptr<SessionDelegate> delegate) : Session()
        {
            _m_delegate = delegate;
        }
    };
}