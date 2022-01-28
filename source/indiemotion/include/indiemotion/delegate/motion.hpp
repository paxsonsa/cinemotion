#pragma once
#include <indiemotion/context.hpp>

namespace indiemotion
{
	struct MotionDelegate
	{
		virtual void motion_updated(Context ctx) {};
	};
}