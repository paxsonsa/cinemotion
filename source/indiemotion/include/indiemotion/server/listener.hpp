#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/logging.hpp>
#include <indiemotion/server/connection.hpp>

namespace indiemotion {

    void fail(beast::error_code ec, char const *what) {
        spdlog::error(fmt::format("{}: {}", what, ec.message()));
    }

    /**
     * TCP Connection Listener
     *
     * A class used to being accepting TCP Connections on a specific tcp::endpoint
     * using the io_context for operations.
     *
     */
    class SessionConnectionListener : public std::enable_shared_from_this<SessionConnectionListener> {
        logging::Logger _logger = logging::get_logger("com.indiemotion.listener");
        asio::io_context &_io_context;
        tcp::acceptor _acceptor;

    public:
        /**
         * Construct a listen to listen on the given endpoint for TCP connections.
         *
         * This constructor will throw std::runtime_error if there are any problems while configuring
         * the listener.
         *
         * @param io_context The context to execute operations within.
         * @param endpoint A tcp endpoint to accept connections on.
         */
        SessionConnectionListener(asio::io_context &io_context,
                                  tcp::endpoint endpoint) : _io_context(io_context), _acceptor(io_context) {
            beast::error_code ec;

            // Open the acceptor
            _acceptor.open(endpoint.protocol(), ec);
            if (ec) {
                auto msg = fmt::format("failed to open listener's acceptor: {}", ec.message());
                _logger->error(msg);
                throw std::runtime_error(msg);
            }

            // Allow address reuse
            _acceptor.set_option(asio::socket_base::reuse_address(true), ec);
            if (ec) {
                auto msg = fmt::format("failed to configure address reuse: {}", ec.message());
                _logger->error(msg);
                throw std::runtime_error(msg);
            }

            // Bind to the server address
            _acceptor.bind(endpoint, ec);
            if (ec) {
                auto msg = fmt::format("failed to bind endpoint: {}", ec.message());
                _logger->error(msg);
                throw std::runtime_error(msg);
            }

            // Start listening for connections
            _acceptor.listen(
                asio::socket_base::max_listen_connections, ec);
            if (ec) {
                auto msg = fmt::format("failed to start listening: {}", ec.message());
                _logger->error(msg);
                throw std::runtime_error(msg);
            }
        }

        /**
         * Start listening and accepting connections.
         * @param callbacks A set of callbacks that will be invoked during a connections lifecycle.
         */
        void listen(SessionConnectionCallbacks &&callbacks) {
            _acceptor.async_accept(
                asio::make_strand(_io_context),
                beast::bind_front_handler(
                    &SessionConnectionListener::on_accept,
                    shared_from_this(),
                    std::move(callbacks)
                )
            );
        }

    private:
        /**
         * Handle when the async accept is invoked.
         * @param callbacks
         * @param ec
         * @param socket
         */
        void on_accept(SessionConnectionCallbacks &&callbacks,
                       beast::error_code ec,
                       tcp::socket socket) {
            if (ec) {
                _logger->error("encountered error accepting connection: {}", ec.message());
                listen(std::move(callbacks));
            } else {
                std::make_shared<SessionConnection>(_io_context, std::move(socket))->start(std::move(callbacks));
            }
        }
    };
}
