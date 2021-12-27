#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion
{
	/**
	 * An Exception Type that contains information for transport to and from client.
	 */
	struct Exception : public std::exception
	{
		/// The type of error that this exception represents
		std::string type;
		/// A user friendly message that describe the error.
		std::string message;
		/// Mark this Exception as Fatal meaning that the session will shutdown immediately.
		bool is_fatal = false;

		/**
		 * Construct an Exception from the type and message
		 * @param type A string representing the error type.
		 * @param message A user friendly message describing the message.
		 */
		Exception(std::string type, std::string message, bool fatal = false) noexcept
			: type(type), message(message), is_fatal(fatal)
		{
			_m_full_message = type;
			_m_full_message += ": " + message;
		}

		/**
		 * Returns exception string
		 * @return a string which is a combination of the type and message.
		 */
		const char* what() const noexcept
		{
			return _m_full_message.c_str();
		}

	private:
		std::string _m_full_message;
	};

	/**
	 * An exception used by delegates to relay errors to the client user.
	 */
	struct SessionException : Exception
	{
		SessionException(std::string m, bool is_fatal = false) : Exception("SessionException",
			m,
			true)
		{
		};
	};

	/**
	 * An Exception that is used when an unknown exception was caught and thus the session
	 * is going to shut down.
	 */
	struct UnknownFatalException : Exception
	{
		UnknownFatalException() : Exception("UnknownFatalError",
			"Session encountered an unknown fatal error, shutting down.",
			true)
		{
		};
	};

	/**
	 * An Exception when the received message is malformed and not able to be processed.
	 */
	struct MalformedMessageException : Exception
	{
		MalformedMessageException() : Exception("MalformedMessageError",
			"The incoming message is malformed.",
			false)
		{
		};
	};

	/**
     * An Exception that is thrown when an operation on a session cannot happen because the
     * Session needs to be initialized.
     */
	struct SessionUninitializedException : Exception
	{
		SessionUninitializedException() : Exception("SessionUninitializedError",
			"Session must be initialized.")
		{
		};
	};

	/**
	 * An Exception that is thrown when the requested API version is
	 * not supported by the server.
	 */
	struct SessionAPIVersionNotSupportedException : Exception
	{
		SessionAPIVersionNotSupportedException() : Exception("SessionAPIVersionNotSupportedError",
			"request api version is not supported.")
		{
		}
	};

	/**
	 * An Exception that is thrown when the operation requires a camera to be set and it is not set.
	 */
	struct CameraNotSetException : Exception
	{
		CameraNotSetException() : Exception("CameraNotSetError",
			"an active camera must be set.")
		{
		}
	};

	/**
	 * An Exception that is thrown when a given camera ID could not be found.
	 */
	struct CameraNotFoundException : Exception
	{
		CameraNotFoundException(std::string camera_id) : Exception("CameraNotFoundError",
			"could not find camera with matching id: "
				+ camera_id)
		{
		}
	};
} // namespace indiemotion::errors
