// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.cpp 

*/
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/server/listener.hpp>
#include <indiemotion/server/options.hpp>
#include <indiemotion/server/common.hpp>
#include <indiemotion/session.hpp>

#include <boost/asio.hpp>

namespace net = boost::asio; // from <boost/asio.hpp>

namespace indiemotion {
    class Server {
        asio::io_context _io_context;
        ServerOptions _options;

    public:
        // Default Constructor
        Server(ServerOptions options)
            : _options(std::move(options)) {};

        void start(ConnectionStartCallback &&cb) {
            auto work = asio::require(_io_context.get_executor(),
                                      asio::execution::outstanding_work.tracked);
            auto const address = net::ip::make_address(_options.address.value_or("0.0.0.0"));
            auto const port = _options.port.value_or(7766);

            // Create Listener and Start its listen routine.
            std::make_shared<Listener>(_io_context,
                                       tcp::endpoint{address, port})->listen(std::move(cb));
            fmt::print("listening on: ws://{}:{}\n", address.to_string(), port);
            _io_context.run();
        }
    };

}
