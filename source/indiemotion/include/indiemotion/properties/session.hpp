#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/session/features.hpp>

namespace indiemotion::properties 
{
    struct SessionProperties {
        std::string name;
        std::string apiVersion;
        session::FeatureSet features;

        SessionProperties() = default;
        SessionProperties(std::string name, 
                          std::string apiVersion, 
                          session::FeatureSet features):
            name(name), apiVersion(apiVersion), features(features) {}

    };
}