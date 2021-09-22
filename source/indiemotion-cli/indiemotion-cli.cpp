// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* indiemotion-cli.cpp 

*/
#include <iostream>
#include <memory>
#include <thread>

#include <boost/program_options.hpp>
#include <replxx.hxx>

#include <indiemotion/motion.hpp>
#include <indiemotion/server.hpp>

#include "./repl.hpp"
#include "./session_delegate.hpp"

namespace progopts = boost::program_options;

/**
 * @brief Command Line Options
 * 
 * 
 */
struct cli_options
{
    // Which port should the server use
    int port;
};

class ServerDelegate : public indiemotion::server::ServerDelegate
{
private:
    std::shared_ptr<indiemotion::motion::Session> _m_session;
    std::weak_ptr<ReplWriter> _m_writer;

public:
    // Default Constructor
    ServerDelegate(std::shared_ptr<ReplWriter> writer) : _m_writer(writer){};

    // Copy the resource (copy constructor)
    ServerDelegate(const ServerDelegate &rhs)
    {
        _m_session = rhs._m_session;
        _m_writer = rhs._m_writer;
    }

    // Transfer Ownership (move constructor)
    ServerDelegate(ServerDelegate &&rhs) noexcept
    {
        _m_session = std::exchange(rhs._m_session, nullptr);
        _m_writer = rhs._m_writer;
    }

    // Make type `std::swap`able
    friend void swap(ServerDelegate &a, ServerDelegate &b) noexcept
    {
        a.swap(b);
    }

    // Destructor
    ~ServerDelegate() {}

    // Assignment by Value
    ServerDelegate &operator=(ServerDelegate copy)
    {
        copy.swap(*this);
        return *this;
    }

    void swap(ServerDelegate &rhs) noexcept
    {
        using std::swap;
        swap(_m_session, rhs._m_session);
        swap(_m_writer, rhs._m_writer);
    }

    void on_new_session(std::shared_ptr<indiemotion::motion::Session> new_session)
    {
        // TODO Return Error Code (object) when the session setup works
        _m_session = new_session;
        if (auto writer = _m_writer.lock())
        {
            writer->write("Recieved new session...\n");

            auto session_delegate = std::make_unique<SessionDelegate>(writer);
            _m_session->set_delegate(std::move(session_delegate));
        }
    }
};

bool parse_options(std::shared_ptr<cli_options> options, int argc, const char **argv)
{
    progopts::options_description descriptor{"IndieMotion Debugger CLI"};

    auto port_opt = progopts::value<int>(&options->port)->default_value(8080)->required();

    auto opt = descriptor.add_options();
    opt = opt("help,h", "Print out this help info");
    opt = opt("port,p", port_opt, "Port to register service on.");

    progopts::variables_map var_map;
    progopts::store(progopts::parse_command_line(argc, argv, descriptor), var_map);

    if (var_map.count("help"))
    {
        std::cout << descriptor << "\n";
        return false;
    }

    // Notify must come after dealing with help or it could throw an exception
    progopts::notify(var_map);
    return true;
}

int main(int argc, const char **argv)
{
    auto options = std::make_shared<cli_options>();
    if (not parse_options(options, argc, argv))
    {
        return 1;
    }

    std::cout
        << "Welcome to IndieMotion Debugger\n"
        << "Press 'tab' to view autocompletions\n"
        << "Type '.help' for help\n"
        << "Type '.quit' or '.exit' to exit\n\n"

        << "Starting Server: 0.0.0.0:" << options->port << "\n\n";

    auto repl = std::make_unique<ReplCore>();

    auto server_delegate = std::make_unique<ServerDelegate>(repl->get_writer());
    auto server_options = std::make_unique<indiemotion::server::Options>();
    server_options->address = "0.0.0.0";
    server_options->port = options->port;

    auto server = std::make_shared<indiemotion::server::Server>(std::move(server_options),
                                                                std::move(server_delegate));

    // Make starting server a command 'START'.
    repl->start();
    auto thread = std::thread(&indiemotion::server::Server::start, server);

    // Initialize Session
    // MessageEvent willInitializeSession()
    // void didInitializeSession()
    // -----
    // Send(InitializeCommand) ->
    // Initialization:
    // device_info (os, application, hostname, ip, unique ID)
    // protocol_version (string)
    // supported_features:
    // - video_streaming
    // - multicam
    // - camera creation

    // Commands
    // InitSession - Server/Client
    // EndSession - Server/Client
    // ChangeSetting - Server/Client
    // ReportStatus - Server/Client
    //
    // UpdatePlaybackMode - Server/Client
    // ChangePlayheadPosition - Server/Client
    //
    // EnableMotionCapture - Server/Client
    // DisableMotionCapture - Server/Client
    //

    // Events
    // Events have an origin (request id) and originator.
    // MotionEvent
    // VideoFrameEvent
    // SessionInitializedEvent
    // SessionEndedEvent
    // SettingsChangedEvent
    // PlaybackModeUpdated
    // PlayheadPositionChanged
    // MotionCaptureEnabled
    // MotionCaptureDisabled

    // ServerObserver
    // - observes server events
    // MotionObserver
    // - observes motion based events
    // PlaybackObserver
    // - observes playback events
    // VideoObserver
    // - observes video events

    // CommandQueue
    // - queue commands to be sent to client/server
    // CommandEventQueue
    // - queue command events to be sent to client/server

    // Server Broadcasts itself
    // Server Accepts Connection and Recieves Session
    // Session is Initialized

    thread.join();
    return 0;
}
