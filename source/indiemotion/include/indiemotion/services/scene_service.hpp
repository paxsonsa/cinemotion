#pragma once
#include <indiemotion/errors.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/scene/camera.hpp>
#include <indiemotion/delegate/scene.hpp>

namespace indiemotion
{
	struct SceneService {
		SceneService(std::shared_ptr<Context> ctx, std::shared_ptr<SceneDelegate> delegate): _ctx(ctx), _delegate(delegate) {
			_ctx->scene = SceneContext::create();
		}

		void initialize()
		{
			load_cameras();
			update_active_camera({});
		}

		void process(const Payloads::SceneInfo& info)
		{
			if (info.has_active_camera_name())
			{
				auto name = info.active_camera_name();
				update_active_camera(name);
			} else {
				update_active_camera({});
			}
			update();
		}

		void load_cameras() const
		{
			if (_delegate)
				_ctx->scene.cameras = _delegate->get_scene_cameras();
		}

		void update_active_camera(std::optional<std::string> name)
		{
			if (!name.has_value())
			{
				_ctx->scene.active_camera_name = {};
				return;
			}

			if (!scene_contains_camera_with_name(*_ctx, name.value()))
			{
				throw CameraNotFoundException(name.value());
			}
			_ctx->scene.active_camera_name = name.value();
		}

	private:
		std::shared_ptr<Context> _ctx;
		std::shared_ptr<SceneDelegate> _delegate;

		void update()
		{
			if (_delegate)
			{
				_delegate->scene_updated(*_ctx);
			}
		}
	};
}