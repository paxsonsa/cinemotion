// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* position.hpp */
#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion::motion
{

    struct _XYZContainer
    {
        double x;
        double y;
        double z;

        static _XYZContainer zero()
        {
            auto ctn = _XYZContainer::create(0, 0, 0);
            return ctn;
        }

        static _XYZContainer create(double x, double y, double z)
        {
            auto ctn = _XYZContainer();
            ctn.x = x;
            ctn.y = y;
            ctn.z = z;
            return std::move(ctn);
        }
    };

    using MotionTranslation = _XYZContainer;
    using MotionOrientation = _XYZContainer;

    /**
     * @brief Transformation Data
     * 
     */
    struct MotionXForm
    {
        MotionTranslation translation;
        MotionOrientation orientation;

        static MotionXForm zero()
        {
            auto xform = MotionXForm();
            xform.translation = MotionTranslation::zero();
            xform.orientation = MotionOrientation::zero();
            return xform;
        }

        static MotionXForm create(double tx, double ty, double tz,
                                                   double ox, double oy, double oz)
        {
            auto xform = MotionXForm();
            xform.translation = MotionTranslation::create(tx, ty, tz);
            xform.orientation = MotionOrientation::create(ox, oy, oz);
            return xform;
        }
    };
}