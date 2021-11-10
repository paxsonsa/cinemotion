// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* server.cpp 

*/
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/server/listener.hpp>
#include <indiemotion/server/options.hpp>
#include <indiemotion/server/server_delegate.hpp>
#include <indiemotion/session/session.hpp>

#include <boost/asio.hpp>

namespace net = boost::asio; // from <boost/asio.hpp>

namespace indiemotion::server
{
    class Server
    {
    private:
        std::unique_ptr<Options> _m_options = nullptr;

    public:
        // Default Constructor
        Server(std::unique_ptr<Options> options)
        {
            _m_options = std::move(options);
        };

        void start()
        {
            // Advertise MDNS
            // Accept connection and start websocket server
            // TODO On return the session is initialized
            // Session initializes by send SESSION_INIT command
            // SessionDelegate::on_event_emit()
            // auto session = std::make_shared<SessionController>();
            // _m_delegate->on_new_session(session);
            // session->start();
            auto const threads = 1;
            net::io_context ioContext{threads};

            auto const address = net::ip::make_address(_m_options->address.value_or("0.0.0.0"));
            auto const port = _m_options->port.value_or(7766);

            std::make_shared<Listener>(ioContext, tcp::endpoint{address, port})->run();
            fmt::print("listening on: ws://{}:{}\n", address.to_string(), port);

            ioContext.run();
        }
    };

}
