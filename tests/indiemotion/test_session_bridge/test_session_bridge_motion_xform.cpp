#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/net/message.hpp>

using namespace indiemotion;

struct DummyDispatcher : NetMessageDispatcher {
    std::vector<NetMessage> messages{};

    void dispatch(NetMessage &&message) {
        messages.push_back(std::move(message));
    }
};

struct DummyDelegate : SessionControllerDelegate
{
    bool wasReceivedMotionUpdateCalled = false;
    MotionXForm xform;

    void did_receive_motion_update(MotionXForm m) override
    {
        wasReceivedMotionUpdateCalled = true;
        xform = m;
    }
};


SCENARIO("updating the motion xform successfully")
{
    GIVEN("an activated 'live' session")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        session->initialize();
        Camera c("cam2");
        session->camera_manager->set_active_cameras(c);
        session->set_motion_mode(MotionMode::Live);

        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);

        WHEN("a motion message is processed")
        {
            auto xform = MotionXForm::create(
                1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);

            auto message = net_make_message();
            auto payload = message.mutable_motion_xform();
            auto orientation = payload->mutable_orientation();
            orientation->set_x(xform.orientation.x);
            orientation->set_y(xform.orientation.y);
            orientation->set_z(xform.orientation.z);
            auto translation = payload->mutable_translation();
            translation->set_x(xform.translation.x);
            translation->set_y(xform.translation.y);
            translation->set_z(xform.translation.z);

            bridge.process_message(std::move(message));

            REQUIRE_FALSE(dispatcher->messages.size() > 0);

            THEN("delegate's recieved motion routine should be invoked")
            {
                REQUIRE(delegate->wasReceivedMotionUpdateCalled);
                REQUIRE(delegate->xform == xform);
            }
        }
    }
}

SCENARIO("updating the motion xform when motion mode is not live or recording")
{
    GIVEN("a fresh active session")
    {
        auto delegate = std::make_shared<DummyDelegate>();
        auto session = std::make_shared<SessionController>(delegate);
        auto dispatcher = std::make_shared<DummyDispatcher>();
        auto bridge = SessionBridge(dispatcher, session);
        auto xform = MotionXForm::create(
            1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
        session->initialize();
        Camera c("cam2");
        session->camera_manager->set_active_cameras(c);

        WHEN("the session's motion mode is off")
        {
            auto message = net_make_message();
            auto payload = message.mutable_motion_xform();
            auto orientation = payload->mutable_orientation();
            orientation->set_x(xform.orientation.x);
            orientation->set_y(xform.orientation.y);
            orientation->set_z(xform.orientation.z);
            auto translation = payload->mutable_translation();
            translation->set_x(xform.translation.x);
            translation->set_y(xform.translation.y);
            translation->set_z(xform.translation.z);

            session->set_motion_mode(MotionMode::Off);
            bridge.process_message(std::move(message));

            THEN("delegate's received motion routine should NOT be invoked")
            {
                REQUIRE(dispatcher->messages.size() == 0); // TODO Check Error
                REQUIRE_FALSE(delegate->wasReceivedMotionUpdateCalled);
                REQUIRE_FALSE(delegate->xform == xform);
            }
        }

        WHEN("the session's motion mode is live")
        {
            auto message = net_make_message();
            auto payload = message.mutable_motion_xform();
            auto orientation = payload->mutable_orientation();
            orientation->set_x(xform.orientation.x);
            orientation->set_y(xform.orientation.y);
            orientation->set_z(xform.orientation.z);
            auto translation = payload->mutable_translation();
            translation->set_x(xform.translation.x);
            translation->set_y(xform.translation.y);
            translation->set_z(xform.translation.z);

            session->set_motion_mode(MotionMode::Live);
            bridge.process_message(std::move(message));


            THEN("delegate's received motion routine should be invoked")
            {
                REQUIRE(dispatcher->messages.size() == 0);
                REQUIRE(delegate->wasReceivedMotionUpdateCalled);
                REQUIRE(delegate->xform == xform);
            }
        }

        WHEN("the session's motion mode is recording")
        {
            auto message = net_make_message();
            auto payload = message.mutable_motion_xform();
            auto orientation = payload->mutable_orientation();
            orientation->set_x(xform.orientation.x);
            orientation->set_y(xform.orientation.y);
            orientation->set_z(xform.orientation.z);
            auto translation = payload->mutable_translation();
            translation->set_x(xform.translation.x);
            translation->set_y(xform.translation.y);
            translation->set_z(xform.translation.z);

            session->set_motion_mode(MotionMode::Recording);
            bridge.process_message(std::move(message));


            THEN("delegate's received motion routine should be invoked")
            {
                REQUIRE(dispatcher->messages.size() == 0);
                REQUIRE(delegate->wasReceivedMotionUpdateCalled);
                REQUIRE(delegate->xform == xform);
            }
        }
    }
}