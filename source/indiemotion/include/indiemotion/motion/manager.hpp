#pragma once
#include <indiemotion/motion/mode.hpp>

namespace indiemotion {
    class MotionManager {
    private:
        MotionMode _m_mode = MotionMode::Off;

    public:
        MotionManager() {}

        MotionMode current_mode() { return _m_mode; }
        void set_current_mode(MotionMode m) {
            _m_mode = m;
        }

        bool can_accept_motion_updates() {
            return _m_mode == MotionMode::Live || _m_mode == MotionMode::Recording;
        }
    };
}