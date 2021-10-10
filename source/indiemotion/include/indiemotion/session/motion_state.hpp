// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* motion_state.hpp */
#pragma once

#include <indiemotion/_common.hpp>

namespace indiemotion::motion
{
    /**
     * @brief A simple value for comparing mode values
     * 
     */
    enum class ModeValue
    {
        Off,
        Live,
        Recording
    };

    class ModeController;

    /**
     * @brief Base class for representing a particular motion mode.
     * 
     */
    class Mode
    {
    protected:
        /**
         * @brief Each mode contains a shared reference to the mode controller
         * 
         */
        std::shared_ptr<ModeController> _m_controller;

    public:
        virtual ~Mode() {}

        /**
         * @brief Set the Controller object
         * 
         * @param ctx 
         */
        void setController(std::shared_ptr<ModeController> ctx)
        {
            _m_controller = ctx;
        }

        virtual ModeValue current() const = 0;
        virtual void handleOff() = 0;
        virtual void handleLive() = 0;
        virtual void handleRecord() = 0;
    };

    /**
     * @brief Controls the current motion mode and provides access to the current mode.
     * 
     */
    class ModeController : public std::enable_shared_from_this<ModeController>
    {
    private:
        std::unique_ptr<Mode> _m_state_ptr;

    public:
        ModeController() {}

        /**
         * @brief Helper function to create a controller in Off mode.
         * 
         * @return std::shared_ptr<ModeController> 
         */
        static std::shared_ptr<ModeController> create();

        /**
         * @brief Transition to a new mode.
         * 
         * @tparam T Should be a Mode subclass.
         */
        template <typename T,
                  typename = std::enable_if_t<std::is_base_of_v<Mode, T>>>
        void transitionTo()
        {
            _m_state_ptr = static_unique_pointer_cast<Mode>(std::make_unique<T>());
            _m_state_ptr->setController(shared_from_this());
        }

        /**
         * @brief Returns the current mode value
         * 
         * @return ModeValue 
         */
        ModeValue current() const
        {
            return _m_state_ptr->current();
        }

        /**
         * @brief Transition to off mode
         * 
         */
        void off()
        {
            _m_state_ptr->handleOff();
        }

        /**
         * @brief Transition to live mode
         * 
         */
        void live()
        {
            _m_state_ptr->handleLive();
        }

        /**
         * @brief Transition to recording mode
         * 
         */
        void record()
        {
            _m_state_ptr->handleRecord();
        }

        bool isRecording() const
        {
            return current() == ModeValue::Recording;
        }

        bool isCapturingMotion() const
        {
            return current() > ModeValue::Off;
        }
    };

    /**
     * @brief Represent Live Motion Mode
     * 
     */
    class LiveMode : public Mode
    {
        ModeValue current() const
        {
            return ModeValue::Live;
        }
        void handleOff();
        void handleLive();
        void handleRecord();
    };

    /**
     * @brief Represents Recording Motion Mode
     * 
     */
    class RecordingMode : public Mode
    {
        ModeValue current() const
        {
            return ModeValue::Recording;
        }
        void handleOff();
        void handleLive();
        void handleRecord();
    };

    /**
     * @brief Representing Off (or no) Motion Mode
     * 
     */
    class OffMode : public Mode
    {
        ModeValue current() const
        {
            return ModeValue::Off;
        }
        void handleOff();
        void handleLive();
        void handleRecord();
    };

    /**
     * @brief Create a new controller preconfigured in off mode.
     * 
     * @return std::shared_ptr<ModeController> 
     */
    std::shared_ptr<ModeController> ModeController::create()
    {
        auto controller = std::make_shared<ModeController>();
        controller->transitionTo<OffMode>();
        return std::move(controller);
    }

    void OffMode::handleOff()
    {
    }
    void OffMode::handleLive()
    {
        _m_controller->transitionTo<LiveMode>();
    }
    void OffMode::handleRecord()
    {
        _m_controller->transitionTo<RecordingMode>();
    }

    void LiveMode::handleOff()
    {
        _m_controller->transitionTo<OffMode>();
    }
    void LiveMode::handleLive()
    {
    }
    void LiveMode::handleRecord()
    {
        _m_controller->transitionTo<RecordingMode>();
    }

    void RecordingMode::handleOff()
    {
        _m_controller->transitionTo<OffMode>();
    }
    void RecordingMode::handleLive()
    {
        _m_controller->transitionTo<LiveMode>();
    }
    void RecordingMode::handleRecord()
    {
    }

}