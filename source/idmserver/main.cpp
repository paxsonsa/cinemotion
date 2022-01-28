#include <iostream>
#include <memory>
#include <thread>

#include <boost/program_options.hpp>

#include <indiemotion/server.hpp>
#include <indiemotion/logging.hpp>
#include <indiemotion/context.hpp>
#include <indiemotion/delegates.hpp>
#include <indiemotion/options.hpp>

using namespace indiemotion;
namespace progopts = boost::program_options;

struct ContextDelegate: public SessionDelegate, SceneDelegate, MotionDelegate
{

	void session_updated(Context ctx) override
	{
	}

	void scene_updated(Context ctx) override
	{
	}

	void motion_updated(Context ctx) override
	{
	}

	std::vector<Camera> get_scene_cameras() override
	{
		return std::vector<Camera>();
	}

	void on_shutdown(Context ctx) override
	{
	}

};


/**
 * Command Line Options
 */
struct cli_options {
    // Which port should the server use
    int port;
};

bool parse_options(std::shared_ptr<cli_options> options, int argc, const char **argv) {
    progopts::options_description descriptor{"IndieMotion Debugger CLI"};

    auto port_opt = progopts::value<int>(&options->port)->default_value(8080)->required();

    auto opt = descriptor.add_options();
    opt = opt("help,h", "Print out this help info");
    opt = opt("port,p", port_opt, "Port to register service on.");

    progopts::variables_map var_map;
    progopts::store(progopts::parse_command_line(argc, argv, descriptor), var_map);

    if (var_map.count("help")) {
        std::cout << descriptor << "\n";
        return false;
    }

    // Notify must come after dealing with help or it could throw an exception
    progopts::notify(var_map);
    return true;
}

int main(int argc, const char **argv) {
	logging::set_global_level(spdlog::level::info);
	logging::configure_default_logger("com.indiemotion");

    auto options = std::make_shared<cli_options>();
    if (not parse_options(options, argc, argv)) {
        return 1;
    }

    std::cout
        << "[Welcome to IndieMotion Debug Server]\n"
        << "\n"
        << "This server exists to test app implements of the specification.\n"
        << "see the docs for more information.\n\n"
        << "Starting Server: 0.0.0.0:" << options->port << "\n\n";

	auto delegate = std::make_shared<ContextDelegate>();
	DelegateInfo delegate_info;
	delegate_info.session = delegate;
	delegate_info.scene = delegate;
	delegate_info.motion = delegate;

    Options server_options;
    server_options.address = "0.0.0.0";
    server_options.port = options->port;
	server_options.delegate_info = delegate_info;

	server_options.on_connect = [&]() {};
	server_options.on_disconnect = [&]() {};

    auto server = Server(server_options);
    std::thread thread{[&server]() {
        server.start();
    }};

    thread.join();
    return 0;
}
