#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/delegate/session.hpp>

namespace indiemotion
{

	struct SessionService final
	{
		std::shared_ptr<Context> ctx;

		SessionService(std::shared_ptr<Context> ctx, std::shared_ptr<SessionDelegate> delegate)
			: ctx(ctx), _delegate(delegate)
		{
			ctx->session = SessionContext::create();
		}

		void initialize()
		{
			update();
		}


		/**
         * Initialize the SessionCon
         *
         * This must be called before any operation can be performed on the session
         * to sure the delegate and managers are ready for operations.
         *
         */
		void process(const Payloads::SessionInfo& info)
		{
			ctx->session.name = info.session_name();
			ctx->session.initialized = true;
			update();
		}

		/**
		 * Shutdown the session
		 */
		void shutdown()
		{
			ctx->session.shutdown = true;
			ctx->session.initialized = false;
			if (_delegate)
			{
				_delegate->on_shutdown(*ctx);
			}
			update();
		}

	private:
		std::shared_ptr<SessionDelegate> _delegate;

		void update()
		{
			if (_delegate)
			{
				_delegate->session_updated(*ctx);
			}
		}
	};
}