#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/session/features.hpp>

namespace indiemotion::session 
{
    struct SessionProperties {
        std::string name;
        std::string apiVersion;
        FeatureSet features;

        SessionProperties() = default;
        SessionProperties(std::string name, 
                          std::string apiVersion, 
                          session::FeatureSet features):
            name(name), apiVersion(apiVersion), features(features) {}

        

    };
}