#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/net.hpp>
#include<indiemotion/logging.hpp>

#include <boost/beast/core/ostream.hpp>
#include <google/protobuf/util/json_util.h>

namespace indiemotion {

    /**
     * A callback that is invoked when the connection is initially started.
     *
     * This callback receives a shared pointer to the session controller to have its
     * delegate updated.
     * ```
     *      [&](std::shared_ptr<SessionController> controller) {
     *          auto delegate = std::make_shared<YourDelegateImpl>();
     *          controller->set_delegate(std::move(delegate);
     *      }
     * ```
     */
    using ConnectionStartCallback = std::function<void(std::shared_ptr<SessionController>)>;

    /**
     * A callback that is invoked when the connection was disconnected.
     *
     * This is not typically used by client facing code as the SessionServer
     * provides its own implementation that calls stop.
     *
     */
    using ConnectionDisconnectedCallback = std::function<void()>;

    /**
     * A container for a set of connection related callbacks.
     */
    struct SessionConnectionCallbacks {
        ConnectionStartCallback on_started;
        ConnectionDisconnectedCallback on_disconnect;
    };

    /**
     * Represents the main connection logic for interfacing with a session
     *
     * A connection handles all websocket socket communication and handling after the
     * the initial tcp conneciton is made with the server.
     *
     */
    class SessionConnection : public std::enable_shared_from_this<SessionConnection> {
    private:
        logging::Logger _logger = logging::getLogger("com.indiemotion.server.connection");
        asio::io_context &_io_context;
        websocket::stream<beast::tcp_stream> _websocket;
        beast::flat_buffer _buffer;
        SessionConnectionCallbacks _callbacks;
        std::unique_ptr<SessionBridge> _session_bridge;
        bool stopped = false;

        /**
         * An internal helper structure that is used by the session bridge to
         * dispatch outgoing messages through the connection itself.
         */
        struct ConnectionWriterDispatcher : public NetMessageDispatcher {

            std::function<void(NetMessage &&message)> callback;

            /**
             * Construct the dispatcher with the callback function that will be invoked each time
             * the bridge dispatches a new message.
             * @param f A function that takes an owned NetMessage as the argument.
             */
            ConnectionWriterDispatcher(std::function<void(NetMessage &&message)> f) : callback(f) {}

            /**
             * Implementation of the dispatch routine. This calls the stored callback function.
             * @param message The message that is being dispatched by the bridge.
             */
            void dispatch(NetMessage &&message) override {
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
        explicit SessionConnection(asio::io_context &io_context, tcp::socket socket) : _io_context(io_context),
                                                                                       _websocket(std::move(socket)) {}

        /**
         * Start the connection and begin accepting websocket communications.
         * @param callbacks A set of callbacks to use as the connection status changes.
         */
        void start(SessionConnectionCallbacks &&callbacks) {
            _callbacks = std::move(callbacks);
            asio::dispatch(_websocket.get_executor(),
                           beast::bind_front_handler(
                               &SessionConnection::onRun,
                               shared_from_this()));
        }

    private:
        /**
         * Start the accepting of websocket communications.
         */
        void onRun() {
            // Set suggested timeout settings for the websocket
            _websocket.set_option(
                websocket::stream_base::timeout::suggested(
                    beast::role_type::server));

            // Set a decorator to change the Server of the handshake
            _websocket.set_option(websocket::stream_base::decorator(
                [](websocket::response_type &res) {
                    res.set(http::field::server,
                            std::string(BOOST_BEAST_VERSION_STRING) +
                                " indiemotion-server");
                }));

            // Accept the websocket handshake
            _websocket.async_accept(
                beast::bind_front_handler(
                    &SessionConnection::onAccept,
                    shared_from_this()));
        }

        /**
         * A method triggered when a websocket is accepted (or not)
         *
         * At this stage consider the connection 'started' and the
         * ConnectionCallbacks.on_start() callback is invoked with a fresh session controller.
         *
         * If there is an error the connection exits and then ConnectionCallbacks.on_disconnect() is invoked.
         *
         * @param err a potential error while accepting the
         */
        void onAccept(beast::error_code err) {
            if (err) {
                _callbacks.on_disconnect();
                _logger->error(fmt::format("Connection::onAccept: {}", err.message()));
                return;
            }
            _logger->info("Accepting Connection...");
            auto controller = std::make_shared<SessionController>();
            auto dispatcher = std::make_shared<ConnectionWriterDispatcher>([&](NetMessage &&message) {
                auto os = beast::ostream(_buffer);
                message.SerializeToOstream(&os);
                _buffer.commit(message.ByteSizeLong());
                _websocket.binary(true);
                _websocket.write(_buffer.data());
                _buffer.consume(message.ByteSizeLong());
            });
            _callbacks.on_started(controller);
            _session_bridge = std::make_unique<SessionBridge>(std::move(dispatcher), std::move(controller));
            do_read();
        }

        /**
         * Schedule an async read task.
         */
        void do_read() {
            _websocket.async_read(
                _buffer,
                beast::bind_front_handler(
                    &SessionConnection::on_read,
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
        void on_read(beast::error_code err, std::size_t bytes_transferred) {
            boost::ignore_unused(bytes_transferred);

            if (err) {
                if (err == boost::asio::error::operation_aborted) {
                    _logger->error(fmt::format("on_read(): op aborted - {}", err.message()));
                } else if (err == websocket::error::closed) {
                    _logger->error(fmt::format("on_read(): close - {}", err.message()));
                } else {
                    _logger->error(fmt::format("Connection::on_read: {}", err.message()));
                }

                _logger->error("connection error, shutting down session and stopping server");

                NetMessage m;
                m.mutable_session_shutdown();
                _session_bridge->processMessage(std::move(m));

                _callbacks.on_disconnect();
                stopped = true;
                return;
            }

            std::string text;
            std::ostringstream os;
            os << boost::beast::buffers_to_string(_buffer.data());
            NetMessage message;
            text = os.str();
            message.ParseFromString(text);

            _buffer.consume(_buffer.size());
            _session_bridge->processMessage(std::move(message));

            // Do another read
            do_read();
        }
    };
}