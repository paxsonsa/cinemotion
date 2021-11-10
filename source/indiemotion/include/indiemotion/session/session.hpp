#pragma once

#include <indiemotion/cameras/manager.hpp>
#include <indiemotion/common.hpp>
#include <indiemotion/motion/manager.hpp>
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
        std::shared_ptr<SessionControllerDelegate> _m_delegate = nullptr;

        std::unique_ptr<cameras::CameraManager> _m_camManager = nullptr;
        std::unique_ptr<MotionManager> _m_motionManager = nullptr;

    public:
        Session()
        {
            _m_camManager = std::make_unique<cameras::CameraManager>();
            _m_motionManager = std::make_unique<MotionManager>();
        }

        Session(std::shared_ptr<SessionControllerDelegate> delegate) : Session()
        {
            _m_delegate = delegate;
        }

        // ----------------------------------------------------------------
        // Session Status
        Status status() const { return _m_status; }
        void setStatus(Status status) { _m_status = status; }

        // ----------------------------------------------------------------
        // Cameras List
        std::vector<cameras::Camera> getCameras() const
        {
            if (_m_delegate)
            {
                return _m_delegate->getAvailableCameras();
            }
            return {};
        }

        std::optional<cameras::Camera> getActiveCamera() const
        {
            return _m_camManager->getActiveCamera();
        }

        void setActiveCamera(std::string cameraId)
        {
            auto cameraOpt = _m_delegate->getCameraById(cameraId);
            if (!cameraOpt)
            {
                // TODO Raise an error when camera optional is empty
            }
            auto camera = cameraOpt.value();
            _m_camManager->setActiveCamera(camera);
            if (_m_delegate)
            {
                _m_delegate->didSetActiveCamera(camera);
            }
        }

        // ----------------------------------------------------------------
        // Motion Mode
        void setMotionMode(MotionMode m)
        {
            _m_motionManager->seCurrentMotionMode(m);
            if (_m_delegate)
            {
                _m_delegate->didMotionSetMode(m);
            }
        }

        MotionMode currentMotionMode() const
        {
            return _m_motionManager->currentMotionMode();
        }

        // ----------------------------------------------------------------
        // Motion Operation
        void updateMotionXForm(MotionXForm xform)
        {
            if (_m_motionManager->canAcceptMotionUpdate())
            {
                if (_m_delegate)
                {
                    _m_delegate->receivedMotionUpdate(xform);
                }
            }
        }
    };
}