#pragma once

namespace indiemotion
{
	struct ContextView;

	struct SessionContext
	{
		std::string name = "";
		bool initialized = false;
		bool shutdown = false;

		struct Delegate
		{
			virtual void session_updated(const ContextView& ctx) {}
		};
	};
}