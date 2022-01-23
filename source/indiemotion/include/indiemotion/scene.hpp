#pragma once
#include <indiemotion/scene/camera.hpp>
#include <indiemotion/contexts/context.hpp>

namespace indiemotion
{
	struct SceneDelegate
	{
		virtual std::vector<Camera> get_scene_cameras() = 0;
		virtual void scene_updated(std::shared_ptr<SceneContext const> scene)
		{
		};
	};

	struct SceneManager {
		SceneManager(std::shared_ptr<Context> ctx, std::shared_ptr<SceneDelegate> delegate): _ctx(ctx), _delegate(delegate) {
			_ctx->scene = std::make_unique<SceneContext>();
		}

		std::vector<Camera> cameras() const
		{
			if (_delegate)
			{
				_ctx->scene->cameras = _delegate->get_scene_cameras();
			}
			return _ctx->scene->cameras;
		}

		void update_active_camera(std::optional<std::string> name)
		{
			_ctx->scene->active_camera_name = name;
			update();
		}

	private:
		std::shared_ptr<Context> _ctx;
		std::shared_ptr<SceneDelegate> _delegate;

		void update()
		{
			if (_delegate)
			{
				_delegate->scene_updated(_ctx->scene);
			}
		}
	};
}