// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* motion_state.hpp */
#pragma once

#include <indiemotion/_common.hpp>

namespace indiemotion::motion
{
    enum class Mode
    {
        Off,
        Live,
        Recording
    };

    class ModeController;

    class ModeHandler
    {
    protected:
        std::shared_ptr<ModeController> _m_controller;

    public:
        virtual ~ModeHandler() {}

        void setController(std::shared_ptr<ModeController> ctx)
        {
            _m_controller = ctx;
        }

        virtual Mode current() const = 0;
        virtual void handleOff() = 0;
        virtual void handleLive() = 0;
        virtual void handleRecord() = 0;
    };

    class OffModeHandler;

    class ModeController : public std::enable_shared_from_this<ModeController>
    {
    private:
        std::unique_ptr<ModeHandler> _m_state_ptr;

    public:
        static std::shared_ptr<ModeController> create()
        {
            auto controller = std::make_shared<ModeController>();
            controller->setState(
                static_unique_pointer_cast<ModeHandler>(
                    std::make_unique<OffModeHandler>()));
            return std::move(controller);
        }

        ModeController() {}

        void setState(std::unique_ptr<ModeHandler> m)
        {
            _m_state_ptr = std::move(m);
            _m_state_ptr->setController(shared_from_this());
        }

        Mode current()
        {
            return _m_state_ptr->current();
        }

        void off()
        {
            _m_state_ptr->handleOff();
        }
        void live()
        {
            _m_state_ptr->handleLive();
        }
        void record()
        {
            _m_state_ptr->handleRecord();
        }
    };

    class LiveModeHandler;
    class RecordingModeHandler;

    class OffModeHandler : public ModeHandler
    {
        Mode current() const
        {
            return Mode::Off;
        }
        void handleOff() {}
        void handleLive()
        {
            _m_controller->setState(
                static_unique_pointer_cast<ModeHandler>(
                    std::make_unique<LiveModeHandler>()));
        }
        void handleRecord()
        {
            _m_controller->setState(
                static_unique_pointer_cast<ModeHandler>(
                    std::make_unique<RecordingModeHandler>()));
        }
    };
    class LiveModeHandler : public ModeHandler
    {
        Mode current() const
        {
            return Mode::Live;
        }
        void handleOff() {}
        void handleLive() {}
        void handleRecord() {}
    };
    class RecordingModeHandler : public ModeHandler
    {
        Mode current() const
        {
            return Mode::Recording;
        }
        void handleOff() {}
        void handleLive() {}
        void handleRecord() {}
    };
}