#pragma once
#include <indiemotion/motion/mode.hpp>

namespace indiemotion::motion
{
    class MotionManager
    {
    private:
        MotionMode _m_mode = MotionMode::Off;

    public:
        MotionManager() {}

        MotionMode currentMotionMode() { return _m_mode; }
        void seCurrentMotionMode(MotionMode m)
        {
            _m_mode = m;
        }

        bool canAcceptMotionUpdate()
        {
            return _m_mode == MotionMode::Live || _m_mode == MotionMode::Recording;
        }
    };
}