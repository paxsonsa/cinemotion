#pragma once
#include <indiemotion/session/delegate.hpp>

namespace indiemotion::session
{

    enum class Status
    {
        Offline,
        Initialized,
        Activated,
    };

    class Session
    {
    private:
        Status _m_status = Status::Offline;
        std::shared_ptr<Delegate> _m_delegate = nullptr;

    public:
        Session() {}

        Session(std::shared_ptr<Delegate> delegate) : Session()
        {
            _m_delegate = delegate;
        }

        Status status() const { return _m_status; }
        void setStatus(Status status) { _m_status = status; }

        std::vector<cameras::Camera> getCameras() const 
        {
            if (_m_delegate)
            {
                return _m_delegate->cameras();
            }
            return {};
        }
    };
}