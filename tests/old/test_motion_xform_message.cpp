// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full
// license information.
/* test_motion_postion_message.hpp


*/
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>

#include <indiemotion/common.hpp>
#include <indiemotion/errors.hpp>
#include <indiemotion/messages/base/message.hpp>
#include <indiemotion/messages/motion/xform/payload.hpp>
#include <indiemotion/motion.hpp>
#include <indiemotion/session.hpp>

using namespace indiemotion;

SCENARIO("Inactive Session Fails to Update MotionXForm") {
  GIVEN("a new session") {
    auto manager = session::SessionManager();

    WHEN("the session is inactive") {
      REQUIRE_FALSE(manager.session()->isActive());

      AND_WHEN("the manager tries to process a positoin update message") {
        auto xform = indiemotion::motion::MotionXForm::zero();
        auto payload = std::make_unique<messages::motion::xform::Payload>(xform);
        auto messagePtr =
            indiemotion::messages::base::createMessage(std::move(payload));
        THEN("an inactive session error should be thrown") {
          REQUIRE_THROWS_AS(manager.processMessage(std::move(messagePtr)),
                            indiemotion::errors::SessionError);
        }
      }
    }
  }
}