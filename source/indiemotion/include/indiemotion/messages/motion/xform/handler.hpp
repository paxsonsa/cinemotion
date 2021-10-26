// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full
// license information.
/* handler.hpp */
#pragma once
#include <indiemotion/messages/base/handler.hpp>
#include <indiemotion/messages/motion/xform/payload.hpp>
#include <indiemotion/responses.hpp>
#include <indiemotion/session.hpp>

namespace indiemotion::messages::motion::xform {
class Handler : public base::Handler {
public:
  virtual std::optional<std::unique_ptr<responses::base::Response>>
  handleMessage(std::weak_ptr<session::Session> sessionPtr,
                std::unique_ptr<Message> messagePtr) {

    // TODO Update Session with incoming XForm Payload

    return {};
  }
};
} // namespace indiemotion::messages::motion::xform