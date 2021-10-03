// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* manager.hpp 

*/
#pragma once
#include <spdlog/spdlog.h>

#include <indiemotion/_common.hpp>
#include <indiemotion/session/session.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/messages/init_message.hpp>
#include <indiemotion/messages/handler.hpp>
#include <indiemotion/messages/handler_factory.hpp>

namespace indiemotion::session
{
    class SessionManager
    {
    private:
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

        // std::optional<messages::Message> process_message(messages::Message m)
        // {
        //     auto handler = _m_handler_factory->get_handler(m.get_kind());
        //     return handler.handle_message(_m_session, m);
        // }
    };
}
