#pragma once
#include <cstdint>

namespace indiemotion::session
{

    typedef uint8_t FeatureSet;

    enum class Features: uint8_t
    {
        VideoStreaming = 0x01,
        TrackHistory = 0x02,
        CustomControls = 0x04,
    };

    Features operator|(Features lhs, Features rhs) {
    return static_cast<Features>(
        static_cast<std::underlying_type<Features>::type>(lhs) |
        static_cast<std::underlying_type<Features>::type>(rhs)
    );
}
}