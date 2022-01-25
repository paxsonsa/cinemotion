#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/contexts/motion_context.hpp>
#include <indiemotion/contexts/scene_context.hpp>
#include <indiemotion/contexts/session_context.hpp>

namespace indiemotion
{
	struct ContextView;

	struct Context: std::enable_shared_from_this<Context>
	{
		std::shared_ptr <SessionContext> session;
		std::shared_ptr <SceneContext> scene;
		std::shared_ptr <MotionContext> motion;

		const ContextView view();


	private:
		std::shared_ptr<ContextView> _view;
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

	const ContextView Context::view()
	{
		return ContextView(shared_from_this());
	}


	bool scene_is_active_camera_set(const ContextView& ctx)
	{
		return ctx.scene()->active_camera_name.has_value();
	}

}