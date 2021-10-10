// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* handler.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/messages/acknowledge.hpp>
#include <indiemotion/messages/cameras.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/messages/kind.hpp>
#include <indiemotion/messages/motion/motion.hpp>
#include <indiemotion/responses/base.hpp>
#include <indiemotion/session/session.hpp>

namespace indiemotion::messages::handling
{
    template <class handler_t>
    std::shared_ptr<Handler> _construct()
    {
        return handler_t::make();
    }

    class HandlerFactory
    {
    private:
        std::shared_ptr<Handler> _m_ptr_table[KindCount];

    public:
        HandlerFactory()
        {
            _m_ptr_table[to_underlying(Kind::Acknowledgment)] = _construct<acknowledge::Handler>();
            _m_ptr_table[to_underlying(Kind::ListCameras)] = _construct<listCameras::Handler>();
            _m_ptr_table[to_underlying(Kind::MotionGetMode)] = _construct<motion::get_mode::Handler>();
            _m_ptr_table[to_underlying(Kind::MotionSetMode)] = _construct<motion::set_mode::Handler>();
        };

        std::shared_ptr<Handler> makeHandler(Kind kind)
        {
            return _m_ptr_table[to_underlying(kind)];
        };
    };
}