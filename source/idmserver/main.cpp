#include <iostream>
#include <memory>
#include <thread>
#include <chrono>

#include <boost/program_options.hpp>

#include <indiemotion/server.hpp>

using namespace indiemotion;
namespace progopts = boost::program_options;

struct ContextDelegate: public SessionDelegate, SceneDelegate, MotionDelegate
{
	long _last_time;
	int running_frame_count = 0;
	int current_frame_count = 60;

	ContextDelegate(): _last_time(std::chrono::high_resolution_clock::now().time_since_epoch().count() / 1000000) {}

	void session_updated(Context ctx) override
	{
	}

	void scene_updated(Context ctx) override
	{
	}

	void motion_updated(Context ctx) override
	{
		std::cout << "motion_update:"
			<< " sps: " << poll()
			<< " tx: " << ctx.motion.current_xform.translation.x
			<< " ty: " << ctx.motion.current_xform.translation.y
			<< " tz: " << ctx.motion.current_xform.translation.z
			<< std::endl;
	}

	std::vector<Camera> get_scene_cameras() override
	{
		return std::vector<Camera>{
			Camera("cam1"),
			Camera("cam2")
		};
	}

	void on_shutdown(Context ctx) override
	{
	}

	int poll()
	{
		auto time = std::chrono::high_resolution_clock::now().time_since_epoch().count() / 1000000;
		if ((time - _last_time) >= 1000) {
			_last_time = std::move(time);
			current_frame_count = running_frame_count;
			running_frame_count = 0;
		}
		running_frame_count += 1;
		return current_frame_count;
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
	server_options.disconnect_behavior = DisconnectBehavior::RestartAlways;

	server_options.on_connect = [&]() {};
	server_options.on_disconnect = [&]() {};

    auto server = std::make_shared<Server>(server_options);
    std::thread thread{[&server]() {
        server->start();
    }};

    thread.join();
    return 0;
}
