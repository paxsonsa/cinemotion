// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* handler.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/session/session.hpp>
#include <indiemotion/responses/base.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/messages/acknowledge.hpp>
#include <indiemotion/messages/cameras.hpp>

namespace indiemotion::messages::handling
{
    template<class handler_t>
    std::shared_ptr<Handler> _construct()
    {
        return handler_t::make();
    }

    class HandlerFactory
    {
    private:
        std::map<Kind, std::shared_ptr<Handler>> _m_ptr_table {};

    public:

        HandlerFactory() = default;

        std::shared_ptr<Handler> makeHandler(Kind kind)
        {
            std::shared_ptr<Handler> p_handler;
            p_handler = _m_ptr_table[kind];

            if (p_handler)
            {
                return p_handler;
            }

            switch(kind)
            {
                case Kind::Acknowledgment:
                    p_handler = _construct<acknowledge::Handler>();
                    break;
                case Kind::ListCameras:
                    p_handler = _construct<listCameras::Handler>();       
                    break;
            }

            _m_ptr_table[kind] = p_handler;
            return p_handler;
        };
    };
}