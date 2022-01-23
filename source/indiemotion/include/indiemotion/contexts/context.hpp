#pragma once
#include <indiemotion/contexts/motion_context.hpp>
#include <indiemotion/contexts/scene_context.hpp>
#include <indiemotion/contexts/session_context.hpp>

namespace indiemotion
{

	struct Context
	{
		std::shared_ptr <SessionContext> session;
		std::shared_ptr <SceneContext> scene;
		std::shared_ptr <MotionContext> motion;
	};

	struct ContextView
	{
		ContextView(std::shared_ptr<Context const> c) : _context(c)
		{
		}

		std::shared_ptr<SessionContext const> const session() const
		{
			return _context->session;
		}

		std::shared_ptr<SceneContext const> scene() const
		{
			return _context->scene;
		}

		std::shared_ptr<MotionContext const> const motion() const
		{
			return _context->motion;
		}

	protected:
		std::shared_ptr<Context const> _context;
	};

	std::shared_ptr <Context> make_context()
	{
		auto c = std::make_shared<Context>();
		return c;
	}

}