#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>

namespace indiemotion
{

    typedef std::uint32_t SessionFeatureSet;

    SessionFeatureSet newFeatureSet(std::uint32_t features)
    {
        return SessionFeatureSet(features);
    }

    enum class SessionFeature : std::uint32_t
    {
        VideoStreaming = 0x01,
        TrackHistory = 0x02,
        CustomControls = 0x04,
    };

    SessionFeature operator|(SessionFeature lhs, SessionFeature rhs)
    {
        return static_cast<SessionFeature>(
            static_cast<std::underlying_type<SessionFeature>::type>(lhs) |
            static_cast<std::underlying_type<SessionFeature>::type>(rhs));
    }

    struct SessionProperties : public net::Payload_T
    {
        std::string id;
        std::string apiVersion;
        SessionFeatureSet features;

        SessionProperties(std::string id,
                          std::string apiVersion,
                          SessionFeatureSet features) : id(id), apiVersion(apiVersion), features(features) {}

        net::PayloadType type() const
        {
            return net::PayloadType::SessionInitilization;
        }
    };
}