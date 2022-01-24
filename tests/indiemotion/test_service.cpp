#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/service.hpp>

using namespace indiemotion;

struct DummyDelegate : public SessionDelegate, SceneDelegate
{
	SessionContext session_ctx;
	SceneContext scene_ctx;

	void session_updated(const std::shared_ptr<const SessionContext>& session) override
	{
		session_ctx = *session;
	}

	void scene_updated(const std::shared_ptr<const SceneContext>& scene) override
	{
		scene_ctx = *scene;
	}
	std::vector<Camera> get_scene_cameras() override
	{
		return std::vector<Camera>();
	}
};

struct DummyDispatcher : NetMessageDispatcher
{
	std::vector<Message> messages{};

	void dispatch(Message&& message)
	{
		messages.push_back(std::move(message));
	}
};

SCENARIO("Basic Execution the Session")
{
	GIVEN("a new service object")
	{

		auto delegate = std::make_shared<DummyDelegate>();
		auto dispatcher = std::make_shared<DummyDispatcher>();
		auto service = Service(dispatcher);
		service.init_session_service(delegate);
		service.init_motion_service(nullptr);
		service.init_scene_service(nullptr);

		WHEN("initialized message is processed")
		{
			Message message;
			auto payload = message.mutable_initialize_session();
			auto properties = payload->mutable_session_info();
			properties->set_api_version("1.0");
			properties->set_session_name("some_id");

			service.process_message(std::move(message));

			THEN("session controller status should be activated")
			{
				REQUIRE(delegate->session_ctx.initialized);
			}

			THEN("session controller name should be set")
			{
				REQUIRE(delegate->session_ctx.name == "some_id");
			}

			THEN("the scene info should be sent")
			{
				auto m = dispatcher->messages[0];
				REQUIRE(m.has_scene_info());
			}

			THEN("the motion info should be sent")
			{
				auto m = dispatcher->messages[1];
				REQUIRE(m.has_motion_info());
			}

		}

		dispatcher->messages.clear();

		WHEN("attempting to change motion data to when no camera is active")
		{
			Message message;
			auto payload = message.mutable_motion_info();
			payload->set_status(Payloads::MotionInfo_Status_Live);

			service.process_message(std::move(message));

			THEN("the service should emit in an error in response")
			{
				auto response = dispatcher->messages[0];
				REQUIRE(response.has_error());

				auto error = response.error();
				REQUIRE(error.type() == Payloads::Error_Type_ActiveCameraNotSetError);
			}
		}

		WHEN("attempting to change motion data to when no camera is active")
		{
			Message message;
			auto payload = message.mutable_motion_info();
			payload->set_status(Payloads::MotionInfo_Status_Live);

			service.process_message(std::move(message));

			THEN("the service should emit in an error in response")
			{
				auto response = dispatcher->messages[0];
				REQUIRE(response.has_error());

				auto error = response.error();
				REQUIRE(error.type() == Payloads::Error_Type_ActiveCameraNotSetError);
			}
		}

		WHEN("attempting to change active camera")
		{
			Message message;
			auto payload = message.mutable_scene_info();
			payload->set_active_camera_name("cam1");

			service.process_message(std::move(message));

			THEN("the active camera context should be updated")
			{
				REQUIRE(service.ctx->scene->active_camera_name == "cam1");
			}

			THEN("the delegate scene context should be updated")
			{
				REQUIRE(delegate->scene_ctx.active_camera_name == "cam1");
			}
		}

	}
}

SCENARIO("Making a message from SceneInfo")
{
	GIVEN("A SceneContext")
	{
		auto ctx = std::make_shared<SceneContext>();
		ctx->active_camera_name = "cam1";
		ctx->cameras.push_back(
			Camera("cam1")
		);
		ctx->cameras.push_back(
			Camera("cam2")
		);

		WHEN("A message is generated from the context")
		{
			auto message = make_message_from(ctx);
			auto scene_info = message.scene_info();

			THEN("The scene info should be populated")
			{
				REQUIRE(scene_info.active_camera_name() == "cam1");
				REQUIRE(scene_info.cameras()[0].name() == "cam1");
				REQUIRE(scene_info.cameras()[1].name() == "cam2");
			}
		}
	}
}

SCENARIO("Making a message from MotionInfo")
{
	GIVEN("A MotionContext")
	{
		auto ctx = MotionContext::create();
		ctx->status = MotionStatus::Idle;
		ctx->current_xform = MotionXForm::create(1,2,3,4,5,6);

		WHEN("A message is generate from the context")
		{
			auto message = make_message_from(ctx);
			auto info = message.motion_info();

			THEN("Expect the status to be set, but not the xform.")
			{
				REQUIRE(info.status() == Payloads::MotionInfo_Status_Idle);
				REQUIRE_FALSE(info.has_xform());
			}
		}
	}
}
// TODO Fix Tests and Clear out old things
// TODO Add proper error handling.