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


        void _throw_when_uninitialized() const
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
                _m_delegate->will_start_session();

            camera_manager = std::make_unique<CameraManager>();
            motion_manager = std::make_unique<MotionManager>();
            _m_status = SessionStatus::Initialized;

            if (_m_delegate)
                _m_delegate->did_start_session();
        }

        void set_delegate(std::shared_ptr<SessionControllerDelegate> delegate) {
            _m_delegate = std::move(delegate);
        }

        // ----------------------------------------------------------------
        // Session Status
        SessionStatus status() const { return _m_status; }

        // ----------------------------------------------------------------
        // Session LifeCycle Calls

        /**
         * Shutdown the session
         */
        void shutdown()
        {
            if (_m_delegate)
            {
                _m_delegate->will_shutdown_session();
            }
            _m_status = SessionStatus::Offline;
        }

        /**
         * Get the current list of cameras available
         * @return a list of camera instances
         */
        std::vector<Camera> get_cameras() const
        {
            _throw_when_uninitialized();
            if (_m_delegate)
            {
                return _m_delegate->get_available_cameras();
            }
            return {};
        }

        /**
         * Get the currently active camera for the session
         * @return an instance of a Camera
         */
        std::optional<Camera> get_active_camera() const
        {
            _throw_when_uninitialized();
            return camera_manager->getActiveCamera();
        }

        /**
         * Set the currently active camera to the given name
         *
         * @param camera_name The camera name to use as the active camera
         */
        void set_active_camera(std::string camera_name)
        {
            _throw_when_uninitialized();
            auto cameraOpt = _m_delegate->get_camera_by_name(camera_name);
            if (!cameraOpt)
            {
                throw CameraNotFoundException(camera_name);
            }
            auto camera = cameraOpt.value();
            camera_manager->setActiveCamera(camera);
            if (_m_delegate)
            {
                _m_delegate->did_set_active_camera(camera);
            }
        }

        /**
         * Set the current motion mode
         *
         * @param m the mode to update to
         */
        void set_motion_mode(MotionMode m)
        {
            _throw_when_uninitialized();

            if (!camera_manager->getActiveCamera() && m != MotionMode::Off)
            {
                throw CameraNotSetException();
            }

            motion_manager->set_current_mode(m);
            if (_m_delegate)
            {
                _m_delegate->did_set_motion_mode(m);
            }
        }

        /**
         * Get the current motion mode that is set.
         *
         * @return a motion mode
         */
        MotionMode current_motion_mode() const
        {
            _throw_when_uninitialized();
            return motion_manager->current_mode();
        }

        /**
         * Update the currently active camera's transform.
         *
         * @param xform A set of xform data.
         */
        void update_motion_xform(MotionXForm xform)
        {
            _throw_when_uninitialized();
            if (motion_manager->can_accept_motion_updates())
            {
                if (_m_delegate)
                {
                    _m_delegate->did_receive_motion_update(xform);
                }
            }
        }
    };
}