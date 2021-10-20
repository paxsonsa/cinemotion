// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* handler_factory.hpp */
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/messages/base/handler.hpp>
#include <indiemotion/messages/cameras/list/handler.hpp>
#include <indiemotion/messages/kind.hpp>
#include <indiemotion/messages/motion/set_mode/handler.hpp>
#include <indiemotion/messages/motion/get_mode/handler.hpp>

namespace indiemotion::messages
{
    template <class handler_t>
    std::shared_ptr<base::Handler> _construct()
    {
        return std::make_shared<handler_t>();
    }

    class HandlerFactory
    {
    private:
        std::shared_ptr<base::Handler> _m_ptr_table[KindCount];

    public:
        HandlerFactory()
        {
            // _m_ptr_table[to_underlying(Kind::Acknowledgment)] = _construct<acknowledge::Handler>();
            _m_ptr_table[to_underlying(Kind::ListCameras)] = _construct<cameras::list::Handler>();
            _m_ptr_table[to_underlying(Kind::MotionSetMode)] = _construct<motion::setmode::Handler>();
            _m_ptr_table[to_underlying(Kind::MotionGetMode)] = _construct<motion::getmode::Handler>();
            // _m_ptr_table[to_underlying(Kind::MotionSetMode)] = _construct<motion::set_mode::Handler>();
            // _m_ptr_table[to_underlying(Kind::MotionXForm)] = _construct<motion::xform::Handler>();
        };

        std::shared_ptr<base::Handler> getHandler(Kind kind)
        {
            return _m_ptr_table[to_underlying(kind)];
        };
    };
}
