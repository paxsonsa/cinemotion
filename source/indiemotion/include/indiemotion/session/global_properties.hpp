#pragma once
#include <indiemotion/session/property.hpp>

namespace indiemotion
{
	struct GlobalProperties
	{
		static SessionProperty SessionName() { return std::move(SessionProperty("global.session_name")); }
		static SessionProperty ActiveCameraID() { return std::move(SessionProperty("global.activate_camera_id")); }
		static SessionProperty MotionCaptureMode() { return std::move(SessionProperty("global.motion_capture_mode")); }

		static std::array<SessionProperty, 3> all_properties() {
			return {
				SessionName(),
				ActiveCameraID(),
				MotionCaptureMode(),
			};
		}

		static bool is_global_property(const SessionProperty &property)
		{
			for (auto &prop: all_properties()) {
				if (prop.name() == property.name())
					return true;
			}
			return false;
		}
	};
}