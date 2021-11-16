// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.cpp 

*/
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/server/listener.hpp>
#include <indiemotion/session.hpp>

namespace indiemotion {

    /**
     * A set of options to configure the server
     */
    struct ServerOptions
    {
        /// An address to bind the interface to, defaults to 0.0.0.0
        std::string address = "0.0.0.0";

        /// The port to bind to, defaults to 7766
        unsigned short port = 7766;
    };

    /**
     * A server for accepting and creating session connections
     */
    class SessionServer : public std::enable_shared_from_this<SessionServer> {
        asio::io_context _io_context;
        ServerOptions _options;

    public:
        /**
         * Construct a server with the given server options.
         * @param options
         */
        SessionServer(ServerOptions options)
            : _options(std::move(options)) {};

        /**
         * Start the server, blocks until finished.
         *
         * It is good practice call this in a new thread as this blocks
         * The callback is called with the new session controller, this should be used
         * to configure your runtime delegate for the session.
         *
         * @param on_start_callback A callback to invoke when a new session connection is activated
         */
        void start(ConnectionStartCallback &&on_start_callback) {
            auto work = asio::require(_io_context.get_executor(),
                                      asio::execution::outstanding_work.tracked);
            auto const address = asio::ip::make_address(_options.address);
            auto const port = _options.port;

            SessionConnectionCallbacks callbacks;
            callbacks.on_started = std::move(on_start_callback);
            callbacks.on_disconnect = [&]() {
                stop();
            };

            std::make_shared<SessionConnectionListener>(_io_context,
                                                        tcp::endpoint{address, port})->listen(std::move(callbacks));
            _io_context.run();
        }

        void stop() {
            _io_context.stop();
        }
    };

}
