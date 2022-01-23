#pragma once

namespace indiemotion
{
	struct SessionContext
	{
		std::string name = "";
		bool initialized = false;
		bool shutdown = false;
	};
}