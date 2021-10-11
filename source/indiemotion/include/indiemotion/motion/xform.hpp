// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* position.hpp */
#pragma once
#include <indiemotion/_common.hpp>

namespace indiemotion::motion
{

    struct _XYZContainer
    {
        double x;
        double y;
        double z;

        static std::unique_ptr<_XYZContainer> zero()
        {
            auto ctn = _XYZContainer::create(0, 0, 0);
            return ctn;
        }

        static std::unique_ptr<_XYZContainer> create(double x, double y, double z)
        {
            auto ctn = std::make_unique<_XYZContainer>();
            ctn->x = x;
            ctn->y = y;
            ctn->z = z;
            return ctn;
        }
    };

    using MotionTranslation = _XYZContainer;
    using MotionOrientation = _XYZContainer;

    struct MotionXForm
    {
        std::shared_ptr<MotionTranslation> translation;
        std::shared_ptr<MotionOrientation> orientation;

        static std::unique_ptr<MotionXForm> zero()
        {
            auto ptr = std::make_unique<MotionXForm>();
            ptr->translation = std::move(MotionTranslation::zero());
            ptr->orientation = std::move(MotionOrientation::zero());
            return ptr;
        }

        static std::unique_ptr<MotionXForm> create(double tx, double ty, double tz,
                                                   double ox, double oy, double oz)
        {
            auto ptr = std::make_unique<MotionXForm>();
            ptr->translation = MotionTranslation::create(tx, ty, tz);
            ptr->orientation = MotionOrientation::create(ox, oy, oz);
            return ptr;
        }
    };

    /**
     * @brief Read-0nly view of a motion xform
     * 
     */
    class MotionXFormView
    {
    private:
        std::shared_ptr<MotionXForm> _m_xform;

    public:
        MotionXFormView(std::shared_ptr<MotionXForm> xform) : _m_xform(xform) {}

        double translationX()
        {
            return _m_xform->translation->x;
        }
        double translationY()
        {
            return _m_xform->translation->y;
        }
        double translationZ()
        {
            return _m_xform->translation->z;
        }
        double orientationX()
        {
            return _m_xform->orientation->x;
        }
        double orientationY()
        {
            return _m_xform->orientation->y;
        }
        double orientationZ()
        {
            return _m_xform->orientation->z;
        }

        MotionTranslation translation()
        {
            return *(_m_xform->translation);
        }

        MotionOrientation orientation()
        {
            return *(_m_xform->orientation);
        }
    };
}