#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/session.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/net/message.hpp>

using namespace indiemotion;

struct DummyDispatcher : NetMessageDispatcher
{
	std::vector<Message> messages{};

	void dispatch(Message&& message)
	{
		messages.push_back(std::move(message));
	}
};

struct DummyDelegate : Application
{
	bool wasMotionModeDidUpdateCalled = false;
	MotionMode mode = MotionMode::Idle;

	void did_set_motion_mode(MotionMode m) override
	{
		wasMotionModeDidUpdateCalled = true;
		mode = m;
	}
	std::optional<Camera> get_camera_by_name(std::string name) override
	{
		return Camera(name);
	}
};

SCENARIO("Set Motion Mode Successfully")
{
	GIVEN("an activated session controller")
	{
		auto delegate = std::make_shared<DummyDelegate>();
		auto session = std::make_shared<Session>(delegate);
		auto dispatcher = std::make_shared<DummyDispatcher>();
		auto bridge = SessionService(dispatcher, session);
		session->initialize();

		auto property = SessionProperty(GlobalProperties::ActiveCameraID(), "cam2");
		session->set_session_property(std::move(property));

		WHEN("bridge processes set motion mode=live message")
		{
			auto message = net_make_message();
			auto payload = message.mutable_session_property();
			payload->set_name(GlobalProperties::MotionCaptureMode().name());
			payload->set_int_value(MotionMode::Live);

			bridge.process_message(std::move(message));

			THEN("Ack response should be returned")
			{
				REQUIRE(dispatcher->messages.size() == 1);
				auto response = dispatcher->messages[0];
				REQUIRE(response.has_acknowledge());
			}

			THEN("the motion mode should be updated")
			{
				auto property = GlobalProperties::MotionCaptureMode();
				REQUIRE(session->get_session_property(&property));
				REQUIRE(property.value_int64() == MotionMode::Live);
			}

			THEN("the delegates motion mode did update")
			{
				REQUIRE(delegate->wasMotionModeDidUpdateCalled);
				REQUIRE(delegate->mode == MotionMode::Live);
			}
		}

		WHEN("bridge processes set motion mode=recording description")
		{
			auto message = net_make_message();
			auto payload = message.mutable_session_property();
			payload->set_name(GlobalProperties::MotionCaptureMode().name());
			payload->set_int_value(MotionMode::Recording);

			bridge.process_message(std::move(message));

			THEN("Ack response should be returned")
			{
				REQUIRE(dispatcher->messages.size() == 1);
				auto response = dispatcher->messages[0];
				REQUIRE(response.has_acknowledge());
			}

			THEN("the motion mode should be updated")
			{
				auto property = GlobalProperties::MotionCaptureMode();
				REQUIRE(session->get_session_property(&property));
				REQUIRE(property.value_int64() == MotionMode::Recording);
			}

			THEN("the delegates motion mode did update")
			{
				REQUIRE(delegate->wasMotionModeDidUpdateCalled);
				REQUIRE(delegate->mode == MotionMode::Recording);
			}
		}

		WHEN("bridge processes set motion mode=off description")
		{
			auto message = net_make_message();
			auto payload = message.mutable_session_property();
			payload->set_name(GlobalProperties::MotionCaptureMode().name());
			payload->set_int_value(MotionMode::Recording);

			bridge.process_message(std::move(message));

			THEN("Ack response should be returned")
			{
				REQUIRE(dispatcher->messages.size() == 1);
				auto response = dispatcher->messages[0];
				REQUIRE(response.has_acknowledge());
			}

			THEN("the motion mode should be updated")
			{
				auto property = GlobalProperties::MotionCaptureMode();
				REQUIRE(session->get_session_property(&property));
				REQUIRE(property.value_int64() == MotionMode::Recording);
			}

			THEN("the delegates motion mode did update")
			{
				REQUIRE(delegate->wasMotionModeDidUpdateCalled);
				REQUIRE(delegate->mode == MotionMode::Recording);
			}
		}
	}
}

SCENARIO("Set Motion Mode Fails")
{
	GIVEN("an activated session controller without an active camera configured")
	{
		auto delegate = std::make_shared<DummyDelegate>();
		auto session = std::make_shared<Session>(delegate);
		auto dispatcher = std::make_shared<DummyDispatcher>();
		auto bridge = SessionService(dispatcher, session);
		session->initialize();

		WHEN("bridge processes set motion mode=live description")
		{
			auto message = net_make_message();
			auto payload = message.mutable_session_property();
			payload->set_name(GlobalProperties::MotionCaptureMode().name());
			payload->set_int_value(MotionMode::Live);
			bridge.process_message(std::move(message));

			THEN("a CameraNotSetError should be dispatched")
			{
				REQUIRE(dispatcher->messages.size() == 1);
				auto response = dispatcher->messages[0];
				REQUIRE(response.has_error());
				auto error = response.error();
				REQUIRE(error.type() == message_payloads::Error::ActiveCameraNotSetError);
			}

			THEN("the motion should NOT change")
			{
				auto property = GlobalProperties::MotionCaptureMode();
				REQUIRE(session->get_session_property(&property));
				REQUIRE(property.value_int64() == MotionMode::Idle);
			}
		}

		WHEN("bridge processes set motion mode=recording description")
		{
			auto message = net_make_message();
			auto payload = message.mutable_session_property();
			payload->set_name(GlobalProperties::MotionCaptureMode().name());
			payload->set_int_value(MotionMode::Recording);
			bridge.process_message(std::move(message));

			THEN("a CameraNotSetError should be dispatched")
			{
				REQUIRE(dispatcher->messages.size() == 1);
				auto response = dispatcher->messages[0];
				REQUIRE(response.has_error());
				auto error = response.error();
				REQUIRE(error.type() == message_payloads::Error::ActiveCameraNotSetError);
			}

			THEN("the motion should NOT change")
			{
				auto property = GlobalProperties::MotionCaptureMode();
				REQUIRE(session->get_session_property(&property));
				REQUIRE(property.value_int64() == MotionMode::Idle);
			}
		}
	}
}

SCENARIO("Get Motion Mode Successfully")
{
	GIVEN("an activated session controller")
	{
		auto delegate = std::make_shared<DummyDelegate>();
		auto session = std::make_shared<Session>(delegate);
		auto dispatcher = std::make_shared<DummyDispatcher>();
		auto bridge = SessionService(dispatcher, session);
		session->initialize();
		auto property = SessionProperty(GlobalProperties::ActiveCameraID(), "cam2");
		session->set_session_property(std::move(property));

		WHEN("get mode description is processed")
		{
			auto message = net_make_message();
			auto payload = message.mutable_get_session_property_by_name();
			payload->set_name(GlobalProperties::MotionCaptureMode().name());
			bridge.process_message(std::move(message));

			THEN("a active motion mode description should be dispatched")
			{
				REQUIRE(dispatcher->messages.size() == 1);
				auto response = dispatcher->messages[0];
				REQUIRE(response.has_session_property());
				REQUIRE(response.session_property().int_value() == MotionMode::Idle);
			}
		}
	}
}