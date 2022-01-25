#pragma once
#include <indiemotion/scene/camera.hpp>

namespace indiemotion
{
	struct ContextView;

	struct SceneContext
	{
		std::optional<std::string> active_camera_name;
		std::vector<Camera> cameras = {};

		struct Delegate
		{
			virtual std::vector<Camera> get_scene_cameras() = 0;
			virtual void scene_updated(const ContextView& ctx) {};
		};
	};
}