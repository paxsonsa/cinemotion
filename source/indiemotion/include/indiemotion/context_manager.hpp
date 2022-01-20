#pragma once
#include <indiemotion/context.hpp>

namespace indiemotion
{
	struct ContextManager
	{
		ContextManager(std::unique_ptr<Context> context)
		{
			_context = std::move(context);
		}

		ContextView const context() const
		{
			return ContextView(_context);
		}

		std::shared_ptr<Context> const mutable_context()
		{
			return _context;
		}

	private:
		std::shared_ptr<Context> _context;
	};

}