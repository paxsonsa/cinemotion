#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/service.hpp>

using namespace indiemotion;

struct DummyDelegate : public SessionDelegate, SceneDelegate, MotionDelegate
{
	SessionContext session_ctx;
	SceneContext scene_ctx;
	MotionContext motion_ctx;

	bool on_shutdown_called = false;
	bool get_scene_cameras_called = false;

	std::vector<Camera> get_scene_cameras() override
	{
		get_scene_cameras_called = true;
		return std::vector<Camera>
		{
			Camera{"cam1"},
			Camera{"cam2"}
		};
	}
	void scene_updated(Context ctx) override
	{
		scene_ctx = ctx.scene;
	}
	void session_updated(Context ctx) override
	{
		session_ctx = ctx.session;
	}
	void motion_updated(Context ctx) override
	{
		motion_ctx = ctx.motion;
	}
	void on_shutdown(Context ctx) override
	{
		on_shutdown_called = true;
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

class SessionTestFixture {

protected:
	std::shared_ptr<DummyDelegate> delegate;
	std::shared_ptr<DummyDispatcher> dispatcher;
	Service service;
public:
	SessionTestFixture() : delegate(std::make_shared<DummyDelegate>()), dispatcher(std::make_shared<DummyDispatcher>()), service(Service(dispatcher))  {
		service.init_session_service(delegate);
		service.init_scene_service(delegate);
		service.init_motion_service(delegate);
	}

protected:
	void makeInitialized()
	{
		service.ctx->session.initialized = true;
		service.ctx->session.name = "some_session";
	}

	void makeActive()
	{
		makeInitialized();
		service.ctx->scene.active_camera_name = "cam1";
	}

	void makeLive()
	{
		makeActive();
		service.ctx->motion.status = MotionStatus::Live;
	}
};


TEST_SUITE("Basic Service Lifecycle")
{
	TEST_CASE_FIXTURE(SessionTestFixture, "Initialize a new session")
	{
		Message message;
		auto payload = message.mutable_initialize_session();
		auto properties = payload->mutable_session_info();
		properties->set_api_version("1.0");
		properties->set_session_name("some_id");
		service.process_message(std::move(message));

		REQUIRE(delegate->session_ctx.initialized);
		REQUIRE(delegate->session_ctx.name == "some_id");
		{
			auto m = dispatcher->messages[0];
			REQUIRE(m.has_scene_info());
			REQUIRE(delegate->get_scene_cameras_called);
		}
		{
			auto m = dispatcher->messages[1];
			REQUIRE(m.has_motion_info());
		}
	}


	TEST_CASE_FIXTURE(SessionTestFixture, "Attempting to change motion data when no camera is active fails.")
	{
		makeInitialized();
		Message message;
		auto payload = message.mutable_motion_info();
		payload->set_status(Payloads::MotionInfo_Status_Live);
		service.process_message(std::move(message));

		auto response = dispatcher->messages[0];
		REQUIRE(response.has_error());

		auto error = response.error();
		REQUIRE(error.type() == Payloads::Error_Type_ActiveCameraNotSetError);
	}

	TEST_CASE_FIXTURE(SessionTestFixture, "Successfully change active camera.")
	{
		makeInitialized();

		Message message;
		auto payload = message.mutable_scene_info();
		payload->set_active_camera_name("cam1");
		service.process_message(std::move(message));

		SUBCASE("the active camera context should be updated")
		{
			REQUIRE(service.ctx->scene.active_camera_name == "cam1");
		}

		SUBCASE("the delegate scene context should be updated")
		{
			REQUIRE(delegate->scene_ctx.active_camera_name.value() == "cam1");
		}
	}

	TEST_CASE_FIXTURE(SessionTestFixture, "Successfully change motion status.")
	{
		makeActive();

		Message message;
		auto payload = message.mutable_motion_info();
		payload->set_status(Payloads::MotionInfo_Status_Live);
		service.process_message(std::move(message));

		SUBCASE("the motion status in the motion context should be updated")
		{
			REQUIRE(service.ctx->motion.status == MotionStatus::Live);
		}
		SUBCASE("the motion context delegate should be invoked with the new status")
		{
			REQUIRE(delegate->motion_ctx.status == MotionStatus::Live);
		}
	}

	TEST_CASE_FIXTURE(SessionTestFixture, "Successfully change motion xform when motion status is **NOT** Idle.")
	{
		makeLive();

		Message message;
		auto payload = message.mutable_motion_info();
		payload->set_status(Payloads::MotionInfo_Status_Live);

		auto xform = payload->mutable_xform();

		auto t = xform->mutable_translation();
		t->set_x(1.0);
		t->set_y(2.0);
		t->set_z(3.0);

		auto o = xform->mutable_orientation();
		o->set_x(4.0);
		o->set_y(5.0);
		o->set_z(6.0);

		service.process_message(std::move(message));

		SUBCASE("the motion status in the motion context should be unchanged")
		{
			REQUIRE(service.ctx->motion.status == MotionStatus::Live);
		}
		SUBCASE("the motion xform should update in the context and emitted to the delegate")
		{
			REQUIRE(service.ctx->motion.current_xform.translation.x == 1.0);
			REQUIRE(service.ctx->motion.current_xform.translation.y == 2.0);
			REQUIRE(service.ctx->motion.current_xform.translation.z == 3.0);
			REQUIRE(service.ctx->motion.current_xform.orientation.x == 4.0);
			REQUIRE(service.ctx->motion.current_xform.orientation.y == 5.0);
			REQUIRE(service.ctx->motion.current_xform.orientation.z == 6.0);

			REQUIRE(delegate->motion_ctx.current_xform.translation.x == 1.0);
			REQUIRE(delegate->motion_ctx.current_xform.translation.y == 2.0);
			REQUIRE(delegate->motion_ctx.current_xform.translation.z == 3.0);
			REQUIRE(delegate->motion_ctx.current_xform.orientation.x == 4.0);
			REQUIRE(delegate->motion_ctx.current_xform.orientation.y == 5.0);
			REQUIRE(delegate->motion_ctx.current_xform.orientation.z == 6.0);
		}

		SUBCASE("Setting motion status idle clears xform data.")
		{
			Message m;
			auto p = m.mutable_motion_info();
			p->set_status(Payloads::MotionInfo_Status_Idle);
			service.process_message(std::move(m));

			SUBCASE("the motion status in the motion context should be updated")
			{
				REQUIRE(service.ctx->motion.status == MotionStatus::Idle);
			}
			SUBCASE("the motion context delegate should be invoked with the new status")
			{
				REQUIRE(delegate->motion_ctx.status == MotionStatus::Idle);
			}

			SUBCASE("the current xform should be reset")
			{
				REQUIRE(service.ctx->motion.current_xform.translation.x == 0.0);
				REQUIRE(service.ctx->motion.current_xform.translation.y == 0.0);
				REQUIRE(service.ctx->motion.current_xform.translation.z == 0.0);
				REQUIRE(service.ctx->motion.current_xform.orientation.x == 0.0);
				REQUIRE(service.ctx->motion.current_xform.orientation.y == 0.0);
				REQUIRE(service.ctx->motion.current_xform.orientation.z == 0.0);

				REQUIRE(delegate->motion_ctx.current_xform.translation.x == 0.0);
				REQUIRE(delegate->motion_ctx.current_xform.translation.y == 0.0);
				REQUIRE(delegate->motion_ctx.current_xform.translation.z == 0.0);
				REQUIRE(delegate->motion_ctx.current_xform.orientation.x == 0.0);
				REQUIRE(delegate->motion_ctx.current_xform.orientation.y == 0.0);
				REQUIRE(delegate->motion_ctx.current_xform.orientation.z == 0.0);
			}
		}
	}

	TEST_CASE_FIXTURE(SessionTestFixture, "Shutting down session make session uninitliazed and shutdown=true.")
	{
		Message message;
		message.mutable_shutdown_session();
		service.process_message(std::move(message));

		SUBCASE("the session initialization and shutdown flags should be flipped")
		{
			REQUIRE_FALSE(service.ctx->session.initialized);
			REQUIRE(service.ctx->session.shutdown);
		}

		SUBCASE("the session context should be emitted to the delegate")
		{
			REQUIRE_FALSE(delegate->session_ctx.initialized);
			REQUIRE(delegate->session_ctx.shutdown);
		}

		SUBCASE("the session context delegate's on_shutdown callback should be called")
		{
			REQUIRE(delegate->on_shutdown_called);
		}
	}

}

SCENARIO("Making a message from SceneInfo")
{
	GIVEN("A SceneContext")
	{
		auto ctx = SceneContext();
		ctx.active_camera_name = "cam1";
		ctx.cameras.push_back(
			Camera("cam1")
		);
		ctx.cameras.push_back(
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
		ctx.status = MotionStatus::Idle;
		ctx.current_xform = MotionXForm::create(1,2,3,4,5,6);

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