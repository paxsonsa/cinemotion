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

public:
    // Default Constructor
    ServerDelegate(){};

    // Copy the resource (copy constructor)
    ServerDelegate(const ServerDelegate &rhs) {}

    // Transfer Ownership (move constructor)
    ServerDelegate(ServerDelegate &&rhs) noexcept
    {
        // member = std::exchange(rhs.member, replacevalue);
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
        // using std::swap;
        //swap(member, rhs.member);
    }

    void on_new_session(std::shared_ptr<indiemotion::motion::Session> new_session)
    {
        auto session_delegate = std::make_unique<SessionDelegate>();
        std::cout << "Recieved new session: " << new_session << std::endl;
        _m_session = new_session;
        _m_session->set_delegate(std::move(session_delegate));
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

    auto server_delegate = std::make_unique<ServerDelegate>();
    auto server_options = std::make_unique<indiemotion::server::Options>();
    server_options->address = "0.0.0.0";
    server_options->port = options->port;

    auto server = std::make_shared<indiemotion::server::Server>(std::move(server_options),
                                                                std::move(server_delegate));

    auto thread = std::thread(&indiemotion::server::Server::start, server);

    /*
     *  ------------------------------- 
     *  Setup REPL
     *  -------------------------------
     * x 1) Create REPL Instance and provide instance to server delegate.
     * 2) Use print command for incoming commands
     * 3) Use SessionState, SessionStateReader, and SessionStateWriter
     * 4) When state changes use state observer to track changes (and print usin REPL::display)
     * 5) REPL recieves command and uses SessionEventWriter to write event to WS.
     */
    auto repl = std::make_unique<ReplCore>();
    repl->start();

    thread.join();
    return 0;
}
