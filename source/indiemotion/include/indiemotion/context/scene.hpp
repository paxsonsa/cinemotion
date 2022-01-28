#pragma once
#include <indiemotion/scene/camera.hpp>

namespace indiemotion
{
	struct SceneContext
	{
		std::optional<std::string> active_camera_name;
		std::vector<Camera> cameras = {};

		static SceneContext create() {
			return SceneContext();
		}
	};
}