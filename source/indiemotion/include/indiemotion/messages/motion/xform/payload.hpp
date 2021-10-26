// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full
// license information.
/* payload.hpp */
#pragma once
#include <indiemotion/messages/kind.hpp>
#include <indiemotion/messages/base/payload.hpp>
#include <indiemotion/motion.hpp>
#include <indiemotion/protobuf.hpp>

namespace indiemotion::messages::motion::xform {
class Payload : public base::Payload {
private:
  indiemotion::motion::MotionXForm _m_xform;

public:
  Payload(indiemotion::motion::MotionXForm xform) : _m_xform(xform) {}

  /**
   * @brief Create a new MotionXForm Payload
   *
   * @param rawPayload
   * @return std::unique_ptr<Payload>
   */
  static std::unique_ptr<Payload>
  create(const protobuf::messages::MotionXForm rawPayload) {

    auto xform = indiemotion::motion::MotionXForm::create(
        rawPayload.translation().x(), rawPayload.translation().y(),
        rawPayload.translation().z(), rawPayload.orientation().x(),
        rawPayload.orientation().y(), rawPayload.orientation().z());

    return std::make_unique<Payload>(std::move(xform));
  }

  Kind kind() override
  {
    return Kind::MotionXForm;
  }
};
} // namespace indiemotion::messages::motion::xform