#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion {
    /**
     * @brief A simple value for comparing mode values
     *
     */
    enum MotionStatus {

        /**
         * Idle mode symbolizes when no motion is being transmitted to server
         */
        Idle,

        /**
         * Live mode symbolizes when motion data is being transmitted but not recorded permanently
         */
        Live,

        /**
         * Recording mode symbolizes when motion data is being transmitted and recorded.
         */
        Recording
    };
}