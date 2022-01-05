#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/errors.hpp>
#include <indiemotion/motion.hpp>
#include <indiemotion/application.hpp>
#include <indiemotion/session/property_table.hpp>
#include <indiemotion/session/global_properties.hpp>

namespace indiemotion
{
    enum class SessionStatus
    {
        Offline,
        Initialized,
    };

    class Session
    {
    public:
		std::shared_ptr<PropertyObserverList> property_observer_list = nullptr;
		std::shared_ptr<SessionPropertyTable> property_table = nullptr;

		Session() {}

        Session(std::shared_ptr<Application> delegate)
        {
			_application = delegate;
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
            if (_application)
                _application->will_start_session();

            _m_status = SessionStatus::Initialized;
			property_table = std::make_shared<SessionPropertyTable>();

			// Set Defaults
			auto property = SessionProperty(GlobalProperties::MotionCaptureMode(), MotionMode::Idle);
			property_table->set(std::move(property));

			property = GlobalProperties::ActiveCameraID();
			property_table->set(std::move(property));

			property = SessionProperty(GlobalProperties::SessionName(), "");
			property_table->set(std::move(property));

			property_observer_list = std::make_shared<PropertyObserverList>();
			property_observer_list->observers.push_back(make_property_observer(
				GlobalProperties::ActiveCameraID(),
				std::bind(&Session::_active_camera_changed, this, std::placeholders::_1)
			));

			property_observer_list->observers.push_back(make_property_observer(
				GlobalProperties::MotionCaptureMode(),
				std::bind(&Session::_motion_mode_changed, this, std::placeholders::_1)
			));

            if (_application)
                _application->did_start_session();

        }

		/**
		 * Set the current application for the session.
		 *
		 * The application is the main interface to responding to changes
		 * in the session. As the session receives changes from the input device,
		 * the application is used to complete or notify of those actions.
		 *
		 * @param app
		 */
        void set_application(std::shared_ptr<Application> app) {
			_application = std::move(app);
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
            if (_application)
            {
                _application->will_shutdown_session();
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
            if (_application)
            {
                return _application->get_available_cameras();
            }
            return {};
        }

        /**
         * Update the currently active camera's transform.
         *
         * @param xform A set of xform data.
         */
        void update_motion_xform(MotionXForm xform)
        {
            _throw_when_uninitialized();
            if (can_accept_motion_updates())
            {
                if (_application)
                {
                    _application->did_receive_motion_update(xform);
                }
            }
        }

		/**
		 * Get the value of a given session property.
		 *
		 * The given property object's value is updated if a property is found in the table.
		 *
		 * @param property The session property look up, this instance will have its value populated if found.
		 * @return Whether the value propety was found or not.
		 */
		bool get_session_property(SessionProperty *property)
		{
			return property_table->get(property);
		}

		/**
		 *
		 * @param property
		 */
		void set_session_property(SessionProperty &&property)
		{
			auto new_property = SessionProperty(property.name());
			if (GlobalProperties::is_global_property(property.name()))
			{
				property_observer_list->update(&property);
			}
			else
			{
				_application->will_update_session_property(&property);
			}
			property_table->set(std::move(property));
		}

		bool can_accept_motion_updates()
		{
			auto property = GlobalProperties::MotionCaptureMode();
			if(!property_table->get(&property)) {
				throw ApplicationException("motion mode is not set, this is not expected.", true);
			}
			return property.value_int64() > 0;
		}

	private:
		SessionStatus _m_status = SessionStatus::Offline;
		std::shared_ptr<Application> _application = nullptr;

		void _throw_when_uninitialized() const
		{
			if (_m_status != SessionStatus::Initialized)
			{
				throw APIVersionNotSupportedException();
			}
		}

		void _active_camera_changed(const std::shared_ptr<SessionProperty::Value> value)
		{
			std::string camera_id;
			try {
				camera_id = std::get<std::string>(*value);
			} catch (const std::bad_variant_access &exc) {
				throw SessionPropertyTypeException("active camera must be string value.");
			}

			auto cam = _application->get_camera_by_name(camera_id);
			if (!cam){
				throw CameraNotFoundException(camera_id);
			}
			_application->did_set_active_camera(cam.value());
		}

		void _motion_mode_changed(const std::shared_ptr<SessionProperty::Value> value)
		{
			auto mode_int = std::get<std::int64_t>(*value);
			auto mode = static_cast<MotionMode>(mode_int);
			auto property = GlobalProperties::ActiveCameraID();

			switch(mode)
			{
			case MotionMode::Idle:
				_application->did_set_motion_mode(mode);
				return;
			default:
				if (property_table->get(&property)) {
					_application->did_set_motion_mode(mode);
					return;
				}
			}
			throw ActiveCameraNotSetException();
		}

    };
}