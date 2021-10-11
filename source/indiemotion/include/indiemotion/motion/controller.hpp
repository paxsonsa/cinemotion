// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* controller.hpp */
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/motion/xform.hpp>

namespace indiemotion::motion
{
    class MotionDelegate
    {
    public:
        virtual ~MotionDelegate() {}
        virtual void didUpdate(std::shared_ptr<MotionXForm> xform) = 0;
    };

    class MotionController
    {
    private:
        std::shared_ptr<MotionXForm> _m_xform;
        std::shared_ptr<MotionDelegate> _m_delegate = nullptr;

    public:
        MotionController() : _m_xform(MotionXForm::zero()) {}
        MotionController(std::unique_ptr<MotionDelegate> delegate) : _m_delegate(std::move(delegate)),
                                                                     _m_xform(MotionXForm::zero())

        {
        }

        MotionController(std::shared_ptr<MotionDelegate> delegate) : _m_delegate(delegate),
                                                                     _m_xform(MotionXForm::zero())

        {
        }

        void bindDelegate(std::shared_ptr<MotionDelegate> delegate)
        {
            _m_delegate = delegate;
        }

        void bindDelegate(std::unique_ptr<MotionDelegate> delegate)
        {
            _m_delegate = std::move(delegate);
        }

        void update(std::unique_ptr<MotionXForm> xform)
        {
            std::shared_ptr<MotionXForm> temp = std::move(xform);
            _m_xform.swap(temp);
            if (_m_delegate)
                _m_delegate->didUpdate(_m_xform);
        }

        std::shared_ptr<MotionXForm> xform() const noexcept
        {
            return _m_xform;
        }
    };
}