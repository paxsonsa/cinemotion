#include <iostream>
#include <memory>
#include <thread>

#include <boost/program_options.hpp>

#include <indiemotion/server.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;
namespace progopts = boost::program_options;

/**
 * Command Line Options
 */
struct cli_options
{
    // Which port should the server use
    int port;
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
        << "Welcome to IndieMotion Test Server\n"
        << "Starting Server: 0.0.0.0:" << options->port << "\n\n";

    ServerOptions server_options;
    server_options.address = "0.0.0.0";
    server_options.port = options->port;

    auto server = Server(server_options);
    std::thread thread{[&server](){
        server.start([](std::shared_ptr<SessionController> controller){
            std::cout << "Session Callback Called...." << std::endl;
        });
    }};

    thread.join();
    return 0;
}
