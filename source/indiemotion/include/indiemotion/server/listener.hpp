// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* listener.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/server/common.hpp>
#include <indiemotion/server/connection.hpp>

#include <boost/asio/dispatch.hpp>
#include <boost/asio/strand.hpp>
#include <boost/beast/core.hpp>
#include <boost/beast/websocket.hpp>

namespace beast = boost::beast;         // from <boost/beast.hpp>
namespace http = beast::http;           // from <boost/beast/http.hpp>
namespace websocket = beast::websocket; // from <boost/beast/websocket.hpp>
namespace net = boost::asio;            // from <boost/asio.hpp> 1
using tcp = boost::asio::ip::tcp;       // from <boost/asio/ip/tcp.hpp> 1

namespace indiemotion {

    void fail(beast::error_code ec, char const *what) {
        spdlog::error(fmt::format("{}: {}", what, ec.message()));
    }

    class Listener : public std::enable_shared_from_this<Listener> {
        net::io_context &_io_context;
        tcp::acceptor _m_acceptor;

    public:
        Listener(net::io_context &ioContext,
                 tcp::endpoint endpoint) : _io_context(ioContext), _m_acceptor(ioContext) {
            beast::error_code ec;

            // Open the acceptor
            _m_acceptor.open(endpoint.protocol(), ec);
            if (ec) {
                fail(ec, "open");
                return;
            }

            // Allow address reuse
            _m_acceptor.set_option(net::socket_base::reuse_address(true), ec);
            if (ec) {
                fail(ec, "set_option");
                return;
            }

            // Bind to the server address
            _m_acceptor.bind(endpoint, ec);
            if (ec) {
                fail(ec, "bind");
                return;
            }

            // Start listening for connections
            _m_acceptor.listen(
                net::socket_base::max_listen_connections, ec);
            if (ec) {
                fail(ec, "listen");
                return;
            }
        }

        void listen(SessionConnectionCallbacks &&callbacks) {
            _m_acceptor.async_accept(
                net::make_strand(_io_context),
                beast::bind_front_handler(
                    &Listener::onAccept,
                    shared_from_this(),
                    std::move(callbacks)
                )
            );
        }

    private:
        void onAccept(SessionConnectionCallbacks &&callbacks,
                      beast::error_code ec,
                      tcp::socket socket) {
            if (ec) {
                fail(ec, "onaccept");
            } else {
                // TODO Make Connection;
                // - Launch an HTTP Session and wait for websocket upgrade.
                // - All other HTTP Requests should return bad request
                std::make_shared<SessionConnection>(_io_context, std::move(socket))->start(std::move(callbacks));
            }

            // TODO how to make sure when the connection drops out we continue to listen for new connections
            // Store Connection
        }
    };
}
