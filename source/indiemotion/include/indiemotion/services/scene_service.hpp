#pragma once
#include <indiemotion/net/message.hpp>
#include <indiemotion/scene/camera.hpp>
#include <indiemotion/contexts/context.hpp>
#include <indiemotion/contexts/scene_context.hpp>

namespace indiemotion
{
	struct SceneService {
		SceneService(std::shared_ptr<Context> ctx, std::shared_ptr<SceneContext::Delegate> delegate): _ctx(ctx), _delegate(delegate) {
			_ctx->scene = std::make_unique<SceneContext>();
		}

		void process(const Payloads::SceneInfo& info)
		{
			if (info.has_active_camera_name())
			{
				_ctx->scene->active_camera_name = info.active_camera_name();
			} else {
				_ctx->scene->active_camera_name = {};
			}
			update();
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
			if (_ctx->scene->active_camera_name == name)
				return;

			_ctx->scene->active_camera_name = name;
		}

	private:
		std::shared_ptr<Context> _ctx;
		std::shared_ptr<SceneContext::Delegate> _delegate;

		void update()
		{
			if (_delegate)
			{
				auto view = ContextView(_ctx);
				_delegate->scene_updated(view);
			}
		}
	};
}