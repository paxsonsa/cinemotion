#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <queue>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

struct DummyDelegate : SessionControllerDelegate
{
	std::optional<Camera> get_camera_by_name(std::string name) override
	{
		return Camera(name);
	}
};

struct DummyDispatcher : NetMessageDispatcher
{
	std::queue<Message> messages{};

	void dispatch(Message&& message)
	{
		messages.push(std::move(message));
	}

	void clear()
	{
		messages = {};
	}
};

TEST_CASE("Basic Start Up of Session")
{
	auto delegate = std::make_shared<DummyDelegate>();
	auto session = std::make_shared<Session>(delegate);
	auto dispatcher = std::make_shared<DummyDispatcher>();
	auto bridge = SessionBridge(dispatcher, session);

	// Initialize
	{
		auto message = net_make_message();
		auto payload = message.mutable_initialize_session();
		auto info = payload->mutable_device_info();
		info->set_session_id("session_name");
		info->set_api_version("1.0");

		bridge.process_message(std::move(message));

		REQUIRE(dispatcher->messages.size() == 3);

		// Receive Acknowledge -----------------------------
		auto response = dispatcher->messages.front();
		REQUIRE(response.has_acknowledge());
		dispatcher->messages.pop(); // pop

		// Receive Motion Mode ----------------------------
		response = dispatcher->messages.front();
		REQUIRE(response.has_session_property());
		auto property = response.session_property();
		REQUIRE(property.name() == GlobalProperties::MotionCaptureMode().name());
		REQUIRE(property.int_value() == MotionMode::Idle);
		dispatcher->messages.pop(); // pop

		// Send ACK
		message = net_make_message_with_response_id(response.header().id());
		message.mutable_acknowledge();
		bridge.process_message(std::move(message));

		// Receive Camera List ----------------------------
		response = dispatcher->messages.front();
		REQUIRE(response.has_camera_list());
		dispatcher->messages.pop(); // pop

		// Send ACK
		message = net_make_message_with_response_id(response.header().id());
		message.mutable_acknowledge();
		bridge.process_message(std::move(message));

		REQUIRE(dispatcher->messages.size() == 0);
	}

	// Send Active Camera
	{
		auto message = net_make_message();
		auto payload = message.mutable_session_property();
		payload->set_name(GlobalProperties::ActiveCameraID().name());
		payload->set_string_value("cam1");

		bridge.process_message(std::move(message));

		REQUIRE(dispatcher->messages.size() == 1);

		// Receive Acknowledge -----------------------------
		auto response = dispatcher->messages.front();
		REQUIRE(response.has_acknowledge());
		dispatcher->messages.pop(); // pop
	}

	// TODO Update Motion Mode
	// TODO Update XForm
	// TODO Turn Off Mode
}

// TODO Check Error Handling.