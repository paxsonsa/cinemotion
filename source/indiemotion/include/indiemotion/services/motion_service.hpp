#pragma once
#include <indiemotion/errors.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/motion/status.hpp>
#include <indiemotion/motion/xform.hpp>
#include <indiemotion/delegate/motion.hpp>

namespace indiemotion
{

	struct MotionService
	{
		MotionService(std::shared_ptr<Context> ctx, std::shared_ptr<MotionDelegate> delegate)
			: _ctx(ctx), _delegate(delegate)
		{
			_ctx->motion = MotionContext::create();
			update();
		}

		void initialize()
		{}

		void process(const Payloads::MotionInfo& info)
		{
			MotionStatus new_status = translate_status(info);
			auto cur_status = status(new_status);

			// Idle mode, not motion capture.
			if (cur_status != MotionStatus::Idle)
			{
				extract_xform(info);
			} else {
				reset_xform();
			}
			update();
		}
		void extract_xform(const Payloads::MotionInfo& info)
		{
			auto in_xform = info.xform();
			auto xform = MotionXForm::create(
				in_xform.translation().x(),
				in_xform.translation().y(),
				in_xform.translation().z(),
				in_xform.orientation().x(),
				in_xform.orientation().y(),
				in_xform.orientation().z()
			);
			_ctx->motion.current_xform = xform;
		}

		void reset_xform()
		{
			auto xform = MotionXForm::zero();
			_ctx->motion.current_xform = xform;
		}
		MotionStatus translate_status(const Payloads::MotionInfo& info) const
		{
			MotionStatus new_status;
			switch(info.status())
			{
			case Payloads::MotionInfo_Status_Idle:
				new_status = Idle;
				break;
			case Payloads::MotionInfo_Status_Live:
				new_status = Live;
				break;
			case Payloads::MotionInfo_Status_Recording:
				new_status = Recording;
				break;
			default:
				throw std::runtime_error("Unknown MotionInfo_Status value in MotionService::process");
			}
			return new_status;
		}

		MotionStatus status() const noexcept
		{
			return _ctx->motion.status;
		}

		MotionStatus status(MotionStatus status)
		{
			if (!scene_is_active_camera_set(*_ctx))
			{
				throw ActiveCameraNotSetException();
			}
			_ctx->motion.status = status;
			return status;
		}

	private:
		std::shared_ptr<Context> _ctx;
		std::shared_ptr<MotionDelegate> _delegate;

		void update()
		{
			if (_delegate)
			{
				_delegate->motion_updated(*_ctx);
			}
		}
	};
}