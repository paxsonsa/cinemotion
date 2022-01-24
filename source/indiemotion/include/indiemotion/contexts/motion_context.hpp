#pragma once
#include <indiemotion/motion/status.hpp>
#include <indiemotion/motion/xform.hpp>

namespace indiemotion
{
	struct MotionContext
	{
		MotionStatus status;
		MotionXForm current_xform;

		static std::shared_ptr<MotionContext> create()
		{
			return std::make_shared<MotionContext>();
		}
	};
}