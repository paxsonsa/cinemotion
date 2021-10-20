// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full
// license information.
/* payload.hpp */
#pragma once
#include <indiemotion/messages/base/payload.hpp>
#include <indiemotion/motions/motion.hpp

namespace indiemotion::messages::motion::xform {
class Payload : public base::Payload {
private:
  indiemotion::motion::MotionTranslation _m_translation;
  indiemotion::motion::MotionOrientation _m_orientation;

public:
  Payload(indiemotion::motion::MotionTranslation &&translation,
          indiemotion::motion::MotionOrientation &&orientation)
      : _m_translation(std::move(translation)),
        _m_orientation(std::move(orientation)) {}

  /**
   * @brief Create a new MotionXForm Payload
   *
   * @param rawPayload
   * @return std::unique_ptr<Payload>
   */
  static std::unique_ptr<Payload>
  create(const protobuf::messages::MotionXForm rawPayload) {

    auto translation = indiemotion::motion::MotionTranslation::create(
        rawPayload.translation.x, rawPayload.translation.y,
        rawPayload.translation.z);

    auto orientation = indiemotion::motion::MotionOrientation::create(
        rawPayload.orientation.x, rawPayload.orientation.y,
        rawPayload.orientation.z);

    return std::make_unique<Payload>(std::move(translation),
                                     std::move(orientation));
  }
};
} // namespace indiemotion::messages::motion::xform