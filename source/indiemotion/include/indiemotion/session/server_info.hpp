#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>

namespace indiemotion
{

    typedef std::uint32_t SessionServerFeatureSet;

    SessionServerFeatureSet newFeatureSet(std::uint32_t features)
    {
        return SessionServerFeatureSet(features);
    }

    enum class SessionServerFeature : std::uint32_t
    {
        VideoStreaming = 0x01,
        TrackHistory = 0x02,
        CustomControls = 0x04,
    };

    SessionServerFeature operator|(SessionServerFeature lhs, SessionServerFeature rhs)
    {
        return static_cast<SessionServerFeature>(
            static_cast<std::underlying_type<SessionServerFeature>::type>(lhs) |
            static_cast<std::underlying_type<SessionServerFeature>::type>(rhs));
    }

    struct SessionServerInfo
    {
        std::string apiVersion;
        SessionServerFeatureSet features;

        SessionServerInfo(std::string apiVersion,
                          SessionServerFeatureSet features) : apiVersion(apiVersion), features(features) {}
    };
}