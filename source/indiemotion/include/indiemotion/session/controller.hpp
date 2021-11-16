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


        void _throwWhenUninitialized() const
        {
            if (_m_status != SessionStatus::Initialized)
            {
                throw SessionUninitializedException();
            }
        }

    public:
        std::unique_ptr<MotionManager> motion_manager = nullptr;
        std::unique_ptr<CameraManager> camera_manager = nullptr;

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

            camera_manager = std::make_unique<CameraManager>();
            motion_manager = std::make_unique<MotionManager>();
            _m_status = SessionStatus::Initialized;

            if (_m_delegate)
                _m_delegate->sessionDidStart();
        }

        void set_delegate(std::shared_ptr<SessionControllerDelegate> delegate) {
            _m_delegate = std::move(delegate);
        }

        // ----------------------------------------------------------------
        // Session Status
        SessionStatus status() const { return _m_status; }

        // ----------------------------------------------------------------
        // Session LifeCycle Calls
        void shutdown()
        {
            if (_m_delegate)
            {
                _m_delegate->sessionWillShutdown();
            }
            _m_status = SessionStatus::Offline;
        }

        // ----------------------------------------------------------------
        // Cameras List
        std::vector<Camera> getCameras() const
        {
            _throwWhenUninitialized();
            if (_m_delegate)
            {
                return _m_delegate->getAvailableCameras();
            }
            return {};
        }

        std::optional<Camera> getActiveCamera() const
        {
            _throwWhenUninitialized();
            return camera_manager->getActiveCamera();
        }

        void setActiveCamera(std::string cameraId)
        {
            _throwWhenUninitialized();
            auto cameraOpt = _m_delegate->getCameraById(cameraId);
            if (!cameraOpt)
            {
                throw CameraNotFoundException(cameraId);
            }
            auto camera = cameraOpt.value();
            camera_manager->setActiveCamera(camera);
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


            if (!camera_manager->getActiveCamera() && m != MotionMode::Off)
            {
                throw CameraNotSetException();
            }

            motion_manager->seCurrentMotionMode(m);
            if (_m_delegate)
            {
                _m_delegate->didMotionSetMode(m);
            }
        }

        MotionMode currentMotionMode() const
        {
            _throwWhenUninitialized();
            return motion_manager->currentMotionMode();
        }

        // ----------------------------------------------------------------
        // Motion Operation
        void updateMotionXForm(MotionXForm xform)
        {
            _throwWhenUninitialized();
            if (motion_manager->canAcceptMotionUpdate())
            {
                if (_m_delegate)
                {
                    _m_delegate->receivedMotionUpdate(xform);
                }
            }
        }
    };
}