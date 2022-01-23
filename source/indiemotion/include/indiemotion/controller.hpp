#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/motion.hpp>
#include <indiemotion/scene.hpp>
#include <indiemotion/contexts/context.hpp>

namespace indiemotion
{
	struct SessionDelegate
	{
		virtual void session_updated(std::shared_ptr<SessionContext const> session) {}
	};

	struct DelegateTable {
		std::shared_ptr<SessionDelegate> session_delegate;
		std::shared_ptr<SceneDelegate> scene_delegate;
		std::shared_ptr<MotionDelegate> motion_delegate;
	};

	struct SessionController final
	{
		std::shared_ptr<Context> _ctx;
		std::shared_ptr<SessionDelegate> _delegate;
		std::shared_ptr<SceneManager> _scene;
		std::shared_ptr<MotionManager> _motion;

		SessionController(std::shared_ptr<Context> ctx, DelegateTable d_table): _ctx(ctx), _delegate(d_table.session_delegate) {
			_ctx->session = std::make_shared<SessionContext>();
			_scene = std::make_shared<SceneManager>(ctx, d_table.scene_delegate);
			_motion = std::make_shared<MotionManager>(ctx, d_table.motion_delegate);
		}

		/**
         * Initialize the SessionCon
         *
         * This must be called before any operation can be performed on the session
         * to sure the delegate and managers are ready for operations.
         *
         */
		void initialize(std::string name)
		{
			_ctx->session->name = name;
			_ctx->session->initialized = true;
			update();
		}

		/**
		 * Shutdown the session
		 */
		void shutdown()
		{
			_ctx->session->shutdown = true;
			update();
		}

		const std::shared_ptr<SceneManager> scene()
		{
			return _scene;
		}

		const std::shared_ptr<MotionManager> motion()
		{
			return _motion;
		}

	private:

		void update()
		{
			if (_delegate)
			{
				_delegate->session_updated(_ctx->session);
			}
		}
	};
}