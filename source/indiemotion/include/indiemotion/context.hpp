#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/context/motion.hpp>
#include <indiemotion/context/scene.hpp>
#include <indiemotion/context/session.hpp>

namespace indiemotion
{
	struct Context
	{
		SessionContext session;
		SceneContext scene;
		MotionContext motion;
	};

	std::shared_ptr <Context> make_context()
	{
		auto c = std::make_shared<Context>();
		return c;
	}

	bool scene_is_active_camera_set(const Context& ctx)
	{
		return ctx.scene.active_camera_name.has_value();
	}

	bool scene_contains_camera_with_name(const Context& ctx, std::string name)
	{
		for (auto &cam: ctx.scene.cameras)
		{
			if (cam.name == name)
				return true;
		}
		return false;
	}

}