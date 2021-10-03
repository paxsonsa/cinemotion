// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* manager.hpp 

*/
#pragma once
#include <spdlog/spdlog.h>

#include <indiemotion/_common.hpp>
#include <indiemotion/session/session.hpp>
#include <indiemotion/messages/curator.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/messages/init_message.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/messages/handler_factory.hpp>
#include <indiemotion/messages/ack_message.hpp>

namespace indiemotion::session
{
    class SessionManager
    {
    private:
        std::unique_ptr<messages::Curator> _m_curator;
        std::unique_ptr<messages::MessageHandlerFactory> _m_handler_factory;
        std::shared_ptr<Session> _m_session;
        std::shared_ptr<spdlog::logger> _m_logger;

    public:
        SessionManager()
        {
            _m_handler_factory = std::make_unique<messages::MessageHandlerFactory>();
            _m_session = std::make_shared<Session>();
            _m_logger = spdlog::get("com.apaxson.indiemotion");
        };

        /**
         * @brief Return the current session object
         * 
         * @return std::shared_ptr<Session> 
         */
        std::shared_ptr<Session> session()
        {
            return _m_session;
        }

        /**
         * @brief Initialize the session
         * 
         * @return std::optional<messages::Message> 
         */
        std::optional<messages::Message> initialize()
        {
            try
            {
                _m_session->initialize();
            }
            catch (const std::exception &e)
            {
                _m_logger->error("failed to intialize the session: '{}'", e.what());
                    // TODO return error message (fatal message);
                    return {};
            }
            auto properties = _m_session->properties();
            
            return std::make_optional<messages::InitSessionMessage>(properties);
        }

        /**
         * @brief Return
         * 
         * @param m 
         * @return std::optional<std::shared_ptr<messages::handler::MessageHandler>> 
         */
        std::optional<messages::Message> processMessage(messages::Message m)
        {   
            auto handler = _m_handler_factory->get_handler(m.getKind());
            return handler->handleMessage(_m_session, m);
        }
    };
}
