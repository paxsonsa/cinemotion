// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* xform.hpp */
#pragma once
#include <indiemotion/common.hpp>

namespace indiemotion::motion
{
    struct _XYZContainer
    {
        double x = 0.0f;
        double y = 0.0f;
        double z = 0.0f;

        _XYZContainer() {}
        _XYZContainer(const _XYZContainer &other) : x(other.x), y(other.y), z(other.z) {}
        _XYZContainer(_XYZContainer &&other) : x(std::move(other.x)), y(std::move(other.y)), z(std::move(other.z)) {}

        _XYZContainer &operator=(_XYZContainer &&rhs)
        {
            x = std::move(rhs.x);
            y = std::move(rhs.y);
            z = std::move(rhs.z);

            return *this;
        }

        _XYZContainer &operator=(const _XYZContainer &rhs)
        {
            x = rhs.x;
            y = rhs.y;
            z = rhs.z;

            return *this;
        }

        bool operator==(const _XYZContainer &rhs) const
        {
            return x == rhs.x && y == rhs.y && z == rhs.z;
        }

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

        MotionXForm()
        {
            translation = MotionTranslation::zero();
            orientation = MotionOrientation::zero();
        }

        MotionXForm(const MotionXForm &other)
        {
            translation = other.translation;
            orientation = other.orientation;
        }

        MotionXForm &operator=(const MotionXForm &rhs)
        {
            translation = rhs.translation;
            orientation = rhs.orientation;
            return *this;
        }

        MotionXForm(MotionXForm &&other)
        {
            swap(std::move(other));
        }

        MotionXForm &operator=(MotionXForm &&rhs)
        {
            swap(std::move(rhs));
            return *this;
        }

        bool operator==(const MotionXForm &rhs) const
        {
            return translation == rhs.translation && orientation == rhs.orientation;
        }

        void swap(MotionXForm &&rhs)
        {
            std::swap(translation, rhs.translation);
            std::swap(orientation, rhs.orientation);
        }

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