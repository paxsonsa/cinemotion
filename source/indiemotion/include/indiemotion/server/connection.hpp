// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* http_connection.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/net.hpp>
#include<indiemotion/logging.hpp>

#include <boost/beast/core/ostream.hpp>
#include <google/protobuf/util/json_util.h>

namespace indiemotion {

    using MessageDispatchCallback = std::function<void(NetMessage && message)>;

    struct ConnectionWriterDispatcher : public NetMessageDispatcher {

        MessageDispatchCallback callback;

        ConnectionWriterDispatcher(MessageDispatchCallback f) : callback(f) {}

        void dispatch(NetMessage &&message) override {
            callback(std::move(message));
        }
    };

    using ConnectionStartCallback = std::function<void(std::shared_ptr<SessionController>)>;
    using ConnectionDisconnectedCallback = std::function<void()>;

    struct ConnectionCallbacks {
        ConnectionStartCallback onStarted;
        ConnectionDisconnectedCallback onDisconnect;
    };

    class Connection : public std::enable_shared_from_this<Connection> {
    private:
        logging::Logger logger = logging::getLogger("com.indiemotion.server.connection");
        asio::io_context &_io_context;
        websocket::stream<beast::tcp_stream> _m_websocket;
        beast::flat_buffer _m_buffer;
        ConnectionCallbacks _callbacks;
        std::unique_ptr<SessionBridge> _session_bridge;
        bool stopped = false;

    public:
        explicit Connection(asio::io_context &io_context, tcp::socket socket) : _io_context(io_context),
                                                                                _m_websocket(std::move(socket)) {}

        void start(ConnectionCallbacks &&callbacks) {
            // We need to be executing within a strand to perform async operations
            // on the I/O objects in this session. Although not strictly necessary
            // for single-threaded contexts, this example code is written to be
            // thread-safe by default.
            _callbacks = std::move(callbacks);
            asio::dispatch(_m_websocket.get_executor(),
                           beast::bind_front_handler(
                               &Connection::onRun,
                               shared_from_this()));
        }

    private:
        void onRun() {
            // Set suggested timeout settings for the websocket
            _m_websocket.set_option(
                websocket::stream_base::timeout::suggested(
                    beast::role_type::server));

            // Set a decorator to change the Server of the handshake
            _m_websocket.set_option(websocket::stream_base::decorator(
                [](websocket::response_type &res) {
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

        void onAccept(beast::error_code err) {
            if (err)
                return spdlog::error(fmt::format("Connection::onAccept: {}", err.message()));

            auto controller = std::make_shared<SessionController>();
            auto dispatcher = std::make_shared<ConnectionWriterDispatcher>([&](NetMessage &&message) {
                auto os = beast::ostream(_m_buffer);
                message.SerializeToOstream(&os);
                _m_buffer.commit(message.ByteSizeLong());
                _m_websocket.write(_m_buffer.data());
            });
            _callbacks.onStarted(controller);
            _session_bridge = std::make_unique<SessionBridge>(std::move(dispatcher), std::move(controller));
            do_read();
        }

        void do_read() {
            _m_websocket.async_read(
                _m_buffer,
                beast::bind_front_handler(
                    &Connection::on_read,
                    shared_from_this()));
        }

        void on_read(beast::error_code err, std::size_t bytesTransfered) {
            boost::ignore_unused(bytesTransfered);

            if (err) {
                if (err == boost::asio::error::operation_aborted) {
                    logger->error(fmt::format("on_read(): op aborted - {}", err.message()));
                } else if (err == websocket::error::closed) {
                    logger->error(fmt::format("on_read(): close - {}", err.message()));
                } else {
                    logger->error(fmt::format("Connection::on_read: {}", err.message()));
                }

                logger->error("connection error, shutting down session and stopping server");

                NetMessage m;
                m.mutable_session_shutdown();
                _session_bridge->processMessage(std::move(m));

                _callbacks.onDisconnect();
                stopped = true;
                return;
            }

            // TODO Log Activity to Connection is kept alive
            // keepActive()

            _m_websocket.text(_m_websocket.got_text());
            std::string bufText;
            std::ostringstream os;
            os << boost::beast::buffers_to_string(_m_buffer.data());
            NetMessage message;
            bufText = os.str();
            message.ParseFromString(bufText);

            _m_buffer.consume(_m_buffer.size());
            _session_bridge->processMessage(std::move(message));

            // Do another read
            do_read();
        }
    };
}