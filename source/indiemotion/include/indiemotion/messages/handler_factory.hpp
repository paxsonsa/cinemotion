// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* handler.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/session/session.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/messages/handler.hpp>

#include <indiemotion/messages/ack_message.hpp>

namespace indiemotion::messages
{
    class MessageHandlerFactory
    {

    private:
        std::map<Kind, std::shared_ptr<handler::MessageHandler>> _m_handler_table{};

        std::shared_ptr<handler::MessageHandler> make_handler(Kind kind)
        {
            switch (kind)
            {
            case Kind::Ack: {
                return std::shared_ptr<handler::AckMessageHandler>();
            }
            default:
            {
                std::stringstream err;
                err << "could not construct hander, unknown message kind: " 
                    << KindNameMappings[kind] << std::endl;
                throw std::runtime_error(err.str());
            }
            }
        }

    public:
        std::shared_ptr<handler::MessageHandler> get_handler(Kind kind)
        {
            std::shared_ptr<handler::MessageHandler> handler_ptr;

            try
            {
                handler_ptr = _m_handler_table[kind];
            }
            catch(const std::exception& e)
            {
                _m_handler_table[kind] = make_handler(kind);
                handler_ptr = _m_handler_table[kind];
            }

            return handler_ptr;
        }
    };
}