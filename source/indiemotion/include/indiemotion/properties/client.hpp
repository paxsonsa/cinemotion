#pragma once
#include <indiemotion/_common.hpp>

namespace indiemotion::properties 
{
    struct ClientProperties
    {
        std::string name;
        std::string deviceID;
        std::vector<std::string> supportedAPIVersions;

        ClientProperties() {}

        ClientProperties(std::string name, std::string deviceID, std::vector<std::string> supportedAPIVersions): 
            name(name), deviceID(deviceID), supportedAPIVersions(supportedAPIVersions) {}
    };
}