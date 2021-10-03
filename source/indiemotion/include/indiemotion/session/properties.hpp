#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/session/features.hpp>

namespace indiemotion::session 
{
    struct Properties {
        std::string name;
        std::string apiVersion;
        FeatureSet features;

        Properties() = default;
        Properties(std::string name, 
                          std::string apiVersion, 
                          session::FeatureSet features):
            name(name), apiVersion(apiVersion), features(features) {}

        

    };
}