#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>

namespace indiemotion::session
{

    typedef std::uint32_t FeatureSet;

    FeatureSet newFeatureSet(std::uint32_t features)
    {
        return FeatureSet(features);
    }

    enum class Feature : std::uint32_t
    {
        VideoStreaming = 0x01,
        TrackHistory = 0x02,
        CustomControls = 0x04,
    };

    Feature operator|(Feature lhs, Feature rhs)
    {
        return static_cast<Feature>(
            static_cast<std::underlying_type<Feature>::type>(lhs) |
            static_cast<std::underlying_type<Feature>::type>(rhs));
    }

    struct SessionProperties : public net::Payload_T
    {
        std::string name;
        std::string apiVersion;
        FeatureSet features;

        SessionProperties(std::string name,
                          std::string apiVersion,
                          session::FeatureSet features) : name(name), apiVersion(apiVersion), features(features) {}

        net::PayloadType type() const
        {
            return net::PayloadType::SessionInitilization;
        }
    };
}