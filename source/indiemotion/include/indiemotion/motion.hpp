#pragma once
#include <indiemotion/motion/status.hpp>
#include <indiemotion/motion/xform.hpp>
#include <indiemotion/contexts/context.hpp>

namespace indiemotion
{
	struct MotionDelegate
	{
		virtual void motion_updated(std::shared_ptr<MotionContext const> motion)
		{
		}
	};

	struct MotionManager
	{
		MotionManager(std::shared_ptr<Context> ctx, std::shared_ptr<MotionDelegate> delegate)
			: _ctx(ctx), _delegate(delegate)
		{
			_ctx->motion = std::make_shared<MotionContext>();
			update();
		}

		MotionStatus status() const
		{
			return _ctx->motion->status;
		}

		void status(MotionStatus status)
		{
			_ctx->motion->status = status;
			update();
		}

		void update_xform(MotionXForm&& xform)
		{
			_ctx->motion->current_xform = std::move(xform);
			update();
		}

	private:
		std::shared_ptr<Context> _ctx;
		std::shared_ptr<MotionDelegate> _delegate;

		void update()
		{
			if (_delegate)
			{
				_delegate->motion_updated(_ctx->motion);
			}
		}
	};
}