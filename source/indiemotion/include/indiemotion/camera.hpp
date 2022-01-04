#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion {
    struct Camera {
        std::string name;

        Camera(std::string name) : name(name) {}

        bool operator==(const Camera &rhs) const { return rhs.name == name; }
    }; // namespace indiemotion::cameras
}