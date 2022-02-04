#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/context.hpp>
#include <indiemotion/delegates.hpp>

namespace indiemotion
{
	enum class DisconnectBehavior
	{
		/// Always restart the listener and server when a session disconnects
		RestartAlways,

		/// Terminate the server immediately
		Terminate
	};
/**
 * A set of options to configure the server
 */
	struct Options
	{
		/// An address to bind the interface to, defaults to 0.0.0.0
		std::string address = "0.0.0.0";

		/// The port to bind to, defaults to 7766
		unsigned short port = 7766;

		/// Contains the set of delegate for responding to the context updates
		DelegateInfo delegate_info;

		/// A callback to use when a new session is connected
		std::function<void(void)> on_connect;

		/// A callback to use when a new session is disconnected
		std::function<void(void)> on_disconnect;

		/// The disconnect behavior of the server
		DisconnectBehavior disconnect_behavior;
	};
}