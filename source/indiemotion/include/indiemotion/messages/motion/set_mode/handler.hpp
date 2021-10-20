// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full
// license information.
/* handler.hpp */
#pragma once
#include <indiemotion/messages/base/handler.hpp>
#include <indiemotion/messages/motion/set_mode/payload.hpp>
#include <indiemotion/responses.hpp>

namespace indiemotion::messages::motion::setmode {
class Handler : public base::Handler {
public:
  std::optional<std::unique_ptr<responses::base::Response>>
  handleMessage(std::weak_ptr<session::Session> sessionPtr,
                std::unique_ptr<base::Message> messagePtr) {
    auto payloadPtr = messagePtr->payloadPtrAs<Payload>();
    auto mode = payloadPtr->newMode();
    if (auto session = sessionPtr.lock()) {
      session->updateMotionMode(mode);
      auto payloadPtr =
          std::make_unique<responses::acknowledge::Payload>(true, "mode set.");
      auto ctnPtr = responses::base::createResponse(messagePtr->header()->id(),
                                                    std::move(payloadPtr));
      return ctnPtr;
    }

    // TODO Error (Session Gone)
    return {};
  }
};
} // namespace indiemotion::messages::motion::setmode
