#pragma once
#include <indiemotion/context.hpp>

namespace indiemotion {
	struct SessionDelegate
	{
		virtual void session_updated(Context ctx) {}
		virtual void on_shutdown(Context ctx) {}
	};
};