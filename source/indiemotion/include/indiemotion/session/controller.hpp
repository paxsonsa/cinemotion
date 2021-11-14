#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/errors.hpp>
#include <indiemotion/cameras/manager.hpp>
#include <indiemotion/motion/manager.hpp>
#include <indiemotion/session/delegate.hpp>

namespace indiemotion
{

    enum class SessionStatus
    {
        Offline,
        Initialized,
    };

    class SessionController
    {
    private:
        SessionStatus _m_status = SessionStatus::Offline;
        std::shared_ptr<SessionControllerDelegate> _m_delegate = nullptr;

        std::unique_ptr<cameras::CameraManager> _m_camManager = nullptr;
        std::unique_ptr<MotionManager> _m_motionManager = nullptr;

        void _throwWhenUninitialized() const
        {
            if (_m_status != SessionStatus::Initialized)
            {
                throw SessionUninitializedException();
            }
        }

    public:
        SessionController() {}

        SessionController(std::shared_ptr<SessionControllerDelegate> delegate) : SessionController()
        {
            _m_delegate = delegate;
        }

        /**
         * Initialize the Session
         *
         * This must be called before any operation can be performed on the session
         * to sure the delegate and managers are ready for operations.
         *
         */
        void initialize()
        {
            if (_m_delegate)
                _m_delegate->sessionWillStart();

            _m_camManager = std::make_unique<cameras::CameraManager>();
            _m_motionManager = std::make_unique<MotionManager>();
            _m_status = SessionStatus::Initialized;

            if (_m_delegate)
                _m_delegate->sessionDidStart();
        }

        // ----------------------------------------------------------------
        // Session Status
        SessionStatus status() const { return _m_status; }

        // ----------------------------------------------------------------
        // Session LifeCycle Calls
        void shutdown()
        {
            _throwWhenUninitialized();
            // TODO Close down mangers
            if (_m_delegate)
            {
                _m_delegate->sessionWillShutdown();
            }
            _m_status = SessionStatus::Offline;
        }

        // ----------------------------------------------------------------
        // Cameras List
        std::vector<cameras::Camera> getCameras() const
        {
            _throwWhenUninitialized();
            if (_m_delegate)
            {
                return _m_delegate->getAvailableCameras();
            }
            return {};
        }

        std::optional<cameras::Camera> getActiveCamera() const
        {
            _throwWhenUninitialized();
            return _m_camManager->getActiveCamera();
        }

        void setActiveCamera(std::string cameraId)
        {
            _throwWhenUninitialized();
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
            _throwWhenUninitialized();
            _m_motionManager->seCurrentMotionMode(m);
            if (_m_delegate)
            {
                _m_delegate->didMotionSetMode(m);
            }
        }

        MotionMode currentMotionMode() const
        {
            _throwWhenUninitialized();
            return _m_motionManager->currentMotionMode();
        }

        // ----------------------------------------------------------------
        // Motion Operation
        void updateMotionXForm(MotionXForm xform)
        {
            _throwWhenUninitialized();
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