// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* handler.hpp 

*/
#pragma once
#include <indiemotion/_common.hpp>
#include <indiemotion/session/session.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/messages/handler.hpp>

namespace indiemotion::messages
{
    class MessageHandlerFactory
    {
    public:
        MessageHandler get_handler(Kind kind)
        {
            switch (kind)
            {
            default:
            {
                std::stringstream err;
                err << "could not construct hander, unknown message kind: " 
                    << KindNameMappings[kind] << std::endl;
                throw std::runtime_error(err.str());
                break;
            }
            }
        }
    };
}