#pragma once
#include <indiemotion/context.hpp>
#include <indiemotion/scene/camera.hpp>

namespace indiemotion {
	struct SceneDelegate
	{
		virtual std::vector<Camera> get_scene_cameras() = 0;
		virtual void scene_updated(Context ctx) {};
	};
};