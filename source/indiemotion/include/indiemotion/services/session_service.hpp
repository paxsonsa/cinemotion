#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/services/motion_service.hpp>
#include <indiemotion/services/scene_service.hpp>
#include <indiemotion/contexts/context.hpp>

namespace indiemotion
{

	struct SessionService final
	{
		std::shared_ptr<Context> ctx;

		SessionService(std::shared_ptr<Context> ctx, std::shared_ptr<SessionContext::Delegate> delegate)
			: ctx(ctx), _delegate(delegate)
		{
			ctx->session = std::make_shared<SessionContext>();
		}

		/**
         * Initialize the SessionCon
         *
         * This must be called before any operation can be performed on the session
         * to sure the delegate and managers are ready for operations.
         *
         */
		void initialize(std::string name)
		{
			ctx->session->name = name;
			ctx->session->initialized = true;
			update();
		}

		/**
		 * Shutdown the session
		 */
		void shutdown()
		{
			ctx->session->shutdown = true;
			update();
		}

	private:
		std::shared_ptr<SessionContext::Delegate> _delegate;

		void update()
		{
			if (_delegate)
			{
				auto view = ContextView(ctx);
				_delegate->session_updated(view);
			}
		}
	};
}