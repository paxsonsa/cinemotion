#pragma once
#include <indiemotion/common.hpp>
#include<indiemotion/logging.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/options.hpp>
#include <indiemotion/service.hpp>
#include <indiemotion/context.hpp>

#include <boost/beast/core/ostream.hpp>
#include <google/protobuf/util/json_util.h>

namespace indiemotion
{
	/**
	 * Represents the main connection logic for interfacing with a session
	 *
	 * A connection handles all websocket socket communication and handling after the
	 * the initial tcp conneciton is made with the server.
	 *
	 */
	class Connection : public std::enable_shared_from_this<Connection>
	{
	private:
		logging::Logger _logger = logging::get_logger("com.indiemotion.server.connection");
		asio::io_context& _io_context; // Not used but important to keep alive.
		websocket::stream<beast::tcp_stream> _websocket;
		beast::flat_buffer _read_buffer;
		beast::flat_buffer _write_buffer;
		std::unique_ptr<Service> _service;
		Options _options;

		/**
		 * An internal helper structure that is used by the session bridge to
		 * dispatch outgoing messages through the connection itself.
		 */
		struct Dispatcher : public NetMessageDispatcher
		{
			std::function<void(Message&& message)> callback;

			/**
			 * Construct the dispatcher with the callback function that will be invoked each time
			 * the bridge dispatches a new description.
			 * @param f A function that takes an owned Message as the argument.
			 */
			Dispatcher(std::function<void(Message&& message)> f) : callback(f)
			{
			}

			/**
			 * Implementation of the dispatch routine. This calls the stored callback function.
			 * @param message The description that is being dispatched by the bridge.
			 */
			void dispatch(Message&& message) override
			{
				callback(std::move(message));
			}
		};

	public:
		/**
		 * Construct a new connection using the given io_context and tcp socket.
		 *
		 * The tcp::socket will be upgraded once the start() routine is called.
		 *
		 * @param io_context This is the conext that all operations will be executed within.
		 * @param socket The tcp socket to accept websocket communications on.
		 */
		explicit Connection(asio::io_context& io_context, tcp::socket socket, Options options) :
			_io_context(io_context),
			_websocket(std::move(socket)),
			_options(options)
		{
		}

		/**
		 * Start the connection and begin accepting websocket communications.
		 * @param callbacks A set of callbacks to use as the connection status changes.
		 */
		void start()
		{
			asio::dispatch(_websocket.get_executor(),
				beast::bind_front_handler(
					&Connection::on_run,
					shared_from_this()));
		}

		void disconnect()
		{
			_options.on_disconnect();
			_io_context.stop();
		}

	private:
		/**
		 * Start the accepting of websocket communications.
		 */
		void on_run()
		{
			// Set suggested timeout settings for the websocket
			_websocket.set_option(
				websocket::stream_base::timeout::suggested(
					beast::role_type::server));

			// Set a decorator to change the Server of the handshake
			_websocket.set_option(websocket::stream_base::decorator(
				[](websocket::response_type& res)
				{
					res.set(http::field::server,
						std::string(BOOST_BEAST_VERSION_STRING) +
							" indiemotion-server");
				}));

			// Accept the websocket handshake
			_websocket.async_accept(
				beast::bind_front_handler(
					&Connection::on_accept,
					shared_from_this()));
		}

		/**
		 * A method triggered when a websocket is accepted (or not)
		 *
		 * At this stage consider the connection 'started' and the
		 * Options.on_connect() callback is invoked/.
		 *
		 * @param err a potential error while accepting the
		 */
		void on_accept(beast::error_code err)
		{
			if (err)
			{
				// FIXME: Call disconnect callback
				_logger->error(fmt::format("Connection::on_accept: {}", err.message()));
				return;
			}
			_logger->info("Accepting Connection...");
			auto dispatcher = std::make_shared<Dispatcher>([&](Message&& message)
			{
				auto os = beast::ostream(_write_buffer);
				message.SerializeToOstream(&os);
				_write_buffer.commit(message.ByteSizeLong());
				_websocket.binary(true);

				_websocket.write(_write_buffer.data());
				_write_buffer.clear();
//				_write_buffer.consume(message.ByteSizeLong());
			});
			_service = std::make_unique<Service>(std::move(dispatcher));
			_options.on_connect();

			_logger->info("initializing session context");
			_service->init_session_service(_options.delegate_info.session);

			_logger->info("initializing scene context");
			_service->init_scene_service(_options.delegate_info.scene);

			_logger->info("initializing motion context");
			_service->init_motion_service(_options.delegate_info.motion);

			do_read();
		}

		/**
		 * Schedule an async read task.
		 */
		void do_read()
		{
			_logger->trace("Connection::do_read");
			_websocket.async_read(
				_read_buffer,
				beast::bind_front_handler(
					&Connection::on_read,
					shared_from_this()));
		}

		/**
		 * Triggered when there is a read event
		 *
		 * This function is the 'brains' of the connection communication.
		 * When an error is countered the connection is promptly shutdown and
		 * the on_disconnect() callback is invoked.
		 *
		 * In normal operations, each mesage is read in and handed to the session bridge
		 * for processing synchronously.
		 *
		 * @param err
		 * @param bytes_transferred
		 */
		void on_read(beast::error_code err, std::size_t bytes_transferred)
		{
			_logger->trace("Connection::on_read()");
			boost::ignore_unused(bytes_transferred);

			if (err)
			{
				if (err == boost::asio::error::operation_aborted)
				{
					_logger->error(fmt::format("Connection::on_read [op aborted] - {}", err.message()));
				}
				else if (err == boost::asio::error::timed_out)
				{
					_logger->error("Connection::on_read [timed out]");
				}
				else if (err == websocket::error::closed)
				{
					_logger->error(fmt::format("Connection::on_read [closed] - {}", err.message()));
				}
				else
				{
					_logger->error(fmt::format("Connection::on_read: {}", err.message()));
				}

				disconnect();
				return;
			}

			std::string text;
			std::ostringstream os;
			os << boost::beast::buffers_to_string(_read_buffer.data());
			Message message;
			text = os.str();
			message.ParseFromString(text);

			std::string msg_str;
			google::protobuf::util::MessageToJsonString(message, &msg_str);
			_logger->trace("incoming message: {}", msg_str);
			_read_buffer.consume(_read_buffer.size());

			_service->process_message(std::move(message));

			// Do another read
			do_read();
		}
	};
}