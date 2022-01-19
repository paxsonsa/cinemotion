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

		std::shared_ptr<ContextView const> const context() const
		{
			return _context;
		}

		std::shared_ptr<Context> const mutable_context() const
		{
			return _context;
		}

	private:
		std::shared_ptr<Context> _context;
	};

}