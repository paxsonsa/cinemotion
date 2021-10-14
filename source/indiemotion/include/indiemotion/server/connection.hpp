// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* http_connection.hpp */
#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion::server
{
    class Connection : public std::enable_shared_from_this<Connection>
    {
    private:
        websocket::stream<beast::tcp_stream> _m_websocket;
        beast::flat_buffer _m_buffer;
        // TODO Add boost to common include
        // TODO init class with socket and set up http connection loop
    public:
        explicit Connection(tcp::socket socket) : _m_websocket(std::move(socket))
        {
        }

        /**
         * @brief Start the connection run loop
         * 
         */
        void run()
        {
            // We need to be executing within a strand to perform async operations
            // on the I/O objects in this session. Although not strictly necessary
            // for single-threaded contexts, this example code is written to be
            // thread-safe by default.
            net::dispatch(_m_websocket.get_executor(),
                          beast::bind_front_handler(
                              &Connection::onRun,
                              shared_from_this()));
        }

    private:
        void onRun()
        {
            // Set suggested timeout settings for the websocket
            _m_websocket.set_option(
                websocket::stream_base::timeout::suggested(
                    beast::role_type::server));

            // Set a decorator to change the Server of the handshake
            _m_websocket.set_option(websocket::stream_base::decorator(
                [](websocket::response_type &res)
                {
                    res.set(http::field::server,
                            std::string(BOOST_BEAST_VERSION_STRING) +
                                " indiemotion-server");
                }));

            // Accept the websocket handshake
            _m_websocket.async_accept(
                beast::bind_front_handler(
                    &Connection::onAccept,
                    shared_from_this()));
        }

        void onAccept(beast::error_code err)
        {
            if (err)
                return spdlog::error(fmt::format("Connection::onAccept: {}", err.message()));

            doRead();
        }

        void doRead()
        {
            _m_websocket.async_read(
                _m_buffer,
                beast::bind_front_handler(
                    &Connection::onRead,
                    shared_from_this()));
        }

        void onRead(beast::error_code err, std::size_t bytesTransfered)
        {
            boost::ignore_unused(bytesTransfered);
            // Happens when the timer closes the socket
            if (err == boost::asio::error::operation_aborted)
                // TODO Connection Shutdown
                return;

            // This indicates that the websocket_session was closed
            if (err == websocket::error::closed)
                // TODO Connection Shutdown
                return;

            if (err)
                return spdlog::error(fmt::format("Connection::onRead: {}", err.message()));

            // TODO Log Activity to Connection is kept alive
            // keepActive()

            std::string bufText;
            _m_websocket.text(_m_websocket.got_text());
            std::ostringstream os;
            os << boost::beast::buffers_to_string(_m_buffer.data());
            bufText = os.str();
            fmt::print("read > {}\n", bufText);

            // Do another read
            doRead();
        }
    };
}