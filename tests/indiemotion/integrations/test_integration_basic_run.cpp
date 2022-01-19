#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <queue>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

struct DummyDelegate : Application
{
	DummyDelegate() {}

	std::optional<Camera> get_camera_by_name(std::string name) override
	{
		return Camera(name);
	}

	bool get_available_cameras_called = false;
	std::vector<Camera> get_available_cameras() override
	{
		get_available_cameras_called = true;
		return {
			Camera("cam1"),
			Camera("cam2"),
			Camera("cam3"),
		};
	}

	std::optional<Camera> set_active_camera_value;
	void did_set_active_camera(Camera camera) override
	{
		set_active_camera_value = camera;
	}

	MotionMode did_set_motion_mode_value;
	void did_set_motion_mode(MotionMode m) override
	{
		did_set_motion_mode_value = m;
	}

	std::optional<MotionXForm> received_motion_xform;
	void did_receive_motion_update(MotionXForm m) override
	{
		received_motion_xform = m;
	}

	bool will_shutdown_session_called = false;
	void will_shutdown_session() override
	{
		will_shutdown_session_called = true;
	}

	bool will_start_session_called = false;
	void will_start_session() override
	{
		will_start_session_called = true;
		Application::will_start_session();
	}

	bool did_start_session_called = false;
	void did_start_session() override
	{
		did_start_session_called = true;
		Application::did_start_session();
	}

	std::optional<SessionProperty> updated_session_property_value;
	void will_update_session_property(const SessionProperty* p) override
	{
		updated_session_property_value = p->copy();
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

TEST_CASE("Basic Start Up of SessionCon")
{
	auto delegate = std::make_shared<DummyDelegate>();
	auto session = std::make_shared<SessionCon>(delegate);
	auto dispatcher = std::make_shared<DummyDispatcher>();
	auto bridge = SessionService(dispatcher, session);

	// Initialize
	{
		auto message = net_make_message();
		auto payload = message.mutable_initialize_session();
		auto info = payload->mutable_device_info();
		info->set_session_id("session_name");
		info->set_api_version("1.0");

		bridge.process_message(std::move(message));

		// Test Delegate Calls
		REQUIRE(delegate->will_start_session_called);
		REQUIRE(delegate->did_start_session_called);
		REQUIRE(delegate->did_set_motion_mode_value == MotionMode::Idle);
		REQUIRE(delegate->get_available_cameras_called);

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

		// Check Delegate Calls
		REQUIRE(delegate->set_active_camera_value);
		REQUIRE(delegate->set_active_camera_value.value().name == "cam1");

		// Receive Acknowledge -----------------------------
		REQUIRE(dispatcher->messages.size() == 1);
		auto response = dispatcher->messages.front();
		REQUIRE(response.has_acknowledge());
		dispatcher->messages.pop(); // pop

		REQUIRE(dispatcher->messages.size() == 0); // Ensure Empty Queue
	}

	// Send Motion Mode Update
	{
		auto message = net_make_message();
		auto payload = message.mutable_session_property();
		payload->set_name(GlobalProperties::MotionCaptureMode().name());
		payload->set_int_value(MotionMode::Live);

		bridge.process_message(std::move(message));

		// Check Delegate
		REQUIRE(delegate->did_set_motion_mode_value == MotionMode::Live);

		// Expect Acknowledgement
		REQUIRE(dispatcher->messages.size() == 1);
		auto response = dispatcher->messages.front();
		REQUIRE(response.has_acknowledge());
		dispatcher->messages.pop(); // pop

		REQUIRE(dispatcher->messages.size() == 0); // Ensure Empty Queue
	}

	// Send Motion XForm
	{
		auto message = net_make_message();
		auto payload = message.mutable_input_device_xform();
		auto t = payload->mutable_translation();
		auto o = payload->mutable_orientation();
		t->set_x(1.0f);
		t->set_y(2.0f);
		t->set_z(3.0f);
		o->set_x(4.0f);
		o->set_y(5.0f);
		o->set_z(6.0f);

		bridge.process_message(std::move(message));

		auto xform = delegate->received_motion_xform.value();
		REQUIRE(xform.translation.x == 1.0f);
		REQUIRE(xform.translation.y == 2.0f);
		REQUIRE(xform.translation.z == 3.0f);
		REQUIRE(xform.orientation.x == 4.0f);
		REQUIRE(xform.orientation.y == 5.0f);
		REQUIRE(xform.orientation.z == 6.0f);

		REQUIRE(dispatcher->messages.size() == 0);
	}

	// Send Motion Mode Update
	{
		auto message = net_make_message();
		auto payload = message.mutable_session_property();
		payload->set_name(GlobalProperties::MotionCaptureMode().name());
		payload->set_int_value(MotionMode::Idle);

		bridge.process_message(std::move(message));

		// Check Delegate
		REQUIRE(delegate->did_set_motion_mode_value == MotionMode::Idle);

		REQUIRE(dispatcher->messages.size() == 1);
		auto response = dispatcher->messages.front();
		REQUIRE(response.has_acknowledge());
		dispatcher->messages.pop(); // pop

		REQUIRE(dispatcher->messages.size() == 0); // Ensure Empty Queue
	}
}
