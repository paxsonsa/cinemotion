// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* factory.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/messages/acknowledge.hpp>
#include <indiemotion/messages/cameras.hpp>

namespace indiemotion::messages::handler
{

    template<class handler_t>
    std::shared_ptr<Handler> _construct()
    {
        return handler_t::make();
    }

    class Factory
    {
    private:
        std::map<message::kind, std::shared_ptr<Handler>> _m_ptr_table {};

    public:

        Factory() = default;

        std::shared_ptr<Handler> makeHandler(message::kind kind)
        {
            std::shared_ptr<Handler> p_handler;
            p_handler = _m_ptr_table[kind];

            if (p_handler)
            {
                return p_handler;
            }

            switch(kind)
            {
                case message::kind::Acknowledgment:
                    p_handler = _construct<acknowledge::AckMessageHandler>();
                    break;
                case message::kind::ListCameras:
                    p_handler = _construct<cameras::ListCamerasMessageHandler>();       
                    break;
            }

            _m_ptr_table[kind] = p_handler;
            return p_handler;
        };
    };
} // namespace indiemotion::messages::handler
