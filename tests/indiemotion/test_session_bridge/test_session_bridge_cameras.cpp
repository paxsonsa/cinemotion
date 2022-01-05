#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/common.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/camera.hpp>

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
	bool sessionWillShutdownCalled = false;

	void will_shutdown_session() //override
	{
		sessionWillShutdownCalled = true;
	}
};

SCENARIO("Listing the Cameras")
{
	struct DummyDelegate : Application
	{
		std::vector<Camera> cameraList{
			Camera("cam1"),
			Camera("cam2"),
			Camera("cam3"),
		};

		std::vector<Camera> get_available_cameras() override
		{
			return cameraList;
		}
	};

	GIVEN("a session bridge")
	{
		auto delegate = std::make_shared<DummyDelegate>();
		auto session = std::make_shared<Session>(delegate);
		auto dispatcher = std::make_shared<DummyDispatcher>();
		auto bridge = SessionService(dispatcher, session);
		session->initialize();

		WHEN("bridge processes list camera messages")
		{
			auto message = net_make_message();
			message.mutable_get_camera_list();

			bridge.process_message(std::move(message));

			REQUIRE(dispatcher->messages.size() == 1);
			auto expected = std::move(dispatcher->messages[0]);

			THEN("the delegates camera list should be returned")
			{
				auto camList = expected.camera_list();
				REQUIRE(camList.cameras_size() == delegate->cameraList.size());
				REQUIRE(camList.cameras()[0].id() == "cam1");
				REQUIRE(camList.cameras()[1].id() == "cam2");
				REQUIRE(camList.cameras()[2].id() == "cam3");
			}
		}
	}
}

SCENARIO("Set the Camera Successfully")
{
	struct DummyDelegate : Application
	{

		std::vector<Camera> cameraList{
			Camera("cam1"),
			Camera("cam2"),
			Camera("cam3"),
		};

		std::optional<Camera> camera;

		std::vector<Camera> get_available_cameras() override
		{
			return cameraList;
		}

		std::optional<Camera> get_camera_by_name(std::string id) override
		{
			assert(id == "cam2" && "should not be possible in this test case.");
			return cameraList[1];
		}

		void did_set_active_camera(Camera cam) override
		{
			camera = cam;
		}
	};

	GIVEN("a session bridge")
	{
		auto delegate = std::make_shared<DummyDelegate>();
		auto session = std::make_shared<Session>(delegate);
		session->initialize();

		auto dispatcher = std::make_shared<DummyDispatcher>();
		auto bridge = SessionService(dispatcher, session);

		WHEN("bridge processes set camera messages")
		{
			auto message = net_make_message();
			auto payload = message.mutable_session_property();
			payload->set_name(GlobalProperties::ActiveCameraID().name());
			payload->set_string_value("cam2");
			bridge.process_message(std::move(message));

			THEN("the delegates camera should be set")
			{
				REQUIRE(dispatcher->messages.size() == 1);
				auto response = dispatcher->messages[0];
				REQUIRE(response.has_acknowledge());
				REQUIRE(delegate->camera.value().name == "cam2");
			}
		}
	}
}
