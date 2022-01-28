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
		/// A user-friendly description that describe the error.
		std::string description;
		/// Mark this Exception as Fatal meaning that the session will shutdown immediately.
		bool is_fatal = false;

		/**
		 * Construct an Exception from the type and description
		 * @param type A string representing the error type.
		 * @param message A user friendly description describing the description.
		 */
		Exception(std::string type, std::string message, bool fatal = false) noexcept
			: type(type), description(message), is_fatal(fatal)
		{
			_m_full_message = type;
			_m_full_message += ": " + message;
		}

		/**
		 * Returns exception string
		 * @return a string which is a combination of the type and description.
		 */
		const char* what() const noexcept
		{
			return _m_full_message.c_str();
		}

	private:
		std::string _m_full_message;
	};

	/**
	 * An Exception that is used when an unknown exception was caught and thus the session
	 * is going to shut down.
	 *
	 * IndieMotion Error: UnknownError(is_fatal=true)
	 *
	 */
	struct UnknownFatalException : Exception
	{
		UnknownFatalException() : Exception("UnknownError",
			"SessionCon encountered an unknown fatal error, shutting down.",
			true)
		{
		};
	};

	/**
	 * An exception used by the input device to relay errors to the application.
	 *
	 * IndieMotion Error: InputDeviceError()
	 *
	 */
	struct InputDeviceException : Exception
	{
		InputDeviceException(std::string m, bool is_fatal = false) : Exception("InputDeviceError",
			m,
			true)
		{
		};
	};

	/**
	 * An exception used by application to relay errors to the input device.
	 *
	 * IndieMotion Error: ApplicationError()
	 *
	 */
	struct ApplicationException : Exception
	{
		ApplicationException(std::string m, bool is_fatal = false) : Exception("ApplicationError",
			m,
			true)
		{
		};
	};

	/**
	 * An exception thrown when the user is doing something incorrect or wrong.
	 *
	 * IndieMotion Error: UserError()
	 *
	 */
	struct UserException : Exception
	{
		UserException(std::string m, bool is_fatal = false) : Exception("UserError", m, is_fatal)
		{
		};
	};

	/**
	 * An Exception when the received description is bad or malformed and not able to be processed.
	 *
	 * IndieMotion Error: BadMessageError
	 *
	 */
	struct BadMessageException : Exception
	{
		BadMessageException(std::string m) : Exception("BadMessageError",
			m,
			false)
		{
		};
	};

	/**
	 * An Exception that is thrown when the requested API version is
	 * not supported by the sender.
	 *
	 * IndieMotion Error: APIVersionNotSupportedError
	 *
	 */
	struct APIVersionNotSupportedException : Exception
	{
		APIVersionNotSupportedException() : Exception("APIVersionNotSupportedError",
			"requested api version is not supported.")
		{
		}
	};

	/**
	 * An Exception that is thrown when an operation on a session cannot happen because the
	 * SessionCon needs to be initialized.
	 *
	 * IndieMotion Error: SessionNotInitializedError
	 *
	 */
	struct SessionNotInitializedException : Exception
	{
		SessionNotInitializedException() : Exception("SessionNotInitializedError",
			"SessionCon must be initialized.")
		{
		};
	};

	/**
	 * An Exception that is thrown when the operation requires a camera to be set and it is not set.
	 *
	 * IndieMotion Error: CameraNotSetError
	 */
	struct ActiveCameraNotSetException : Exception
	{
		ActiveCameraNotSetException() : Exception("ActiveCameraNotSetError",
			"an active camera must be set.")
		{
		}
	};

	/**
	 * An Exception that is thrown when a given camera ID could not be found.
	 *
	 * IndieMotion Error: CameraNotFoundError
	 *
	 */
	struct CameraNotFoundException : Exception
	{
		CameraNotFoundException(std::string camera_name) : Exception("CameraNotFoundError",
			"could not find camera with matching name: "
				+ camera_name)
		{
		}
	};

} // namespace indiemotion::errors
