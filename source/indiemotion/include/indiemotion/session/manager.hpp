// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* manager.hpp

*/
#pragma once
#include <spdlog/spdlog.h>

#include <indiemotion/common.hpp>
#include <indiemotion/errors.hpp>
#include <indiemotion/messages/base/wrapper.hpp>
#include <indiemotion/messages/curator.hpp>
#include <indiemotion/messages/handler_factory.hpp>
#include <indiemotion/messages/kind.hpp>
#include <indiemotion/responses.hpp>
#include <indiemotion/responses/base/wrapper.hpp>
#include <indiemotion/session/session.hpp>

namespace indiemotion::session
{
    class SessionManager
    {
    private:
        std::unique_ptr<messages::HandlerFactory> _m_factory;
        std::unique_ptr<messages::Curator> _m_curator;
        std::shared_ptr<Session> _m_session;
        std::shared_ptr<spdlog::logger> _m_logger;

    public:
        SessionManager()
        {
            _m_factory = std::make_unique<messages::HandlerFactory>();
            _m_curator = std::make_unique<messages::Curator>();
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
         * @return std::unique_ptr<messages::response::BaseResponse>
         */
        std::unique_ptr<response::base::Response> initialize()
        {
            std::unique_ptr<responses::base::Response> p_msg;
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
            p_msg = std::make_unique<responses::initialize::Response>(properties);

            // Register a ack callback with the curator
            _m_curator->queue(p_msg->id(), [&]()
                              { _m_session->activate(); });

            return p_msg;
        }

        /**
         * @brief Return
         *
         * @param m
         * @return std::optional<std::shared_ptr<messages::Handler>>
         */
        std::optional<std::unique_ptr<responses::base::Response>> processMessage(std::unique_ptr<messages::base::Wrapper> m)
        {
            // if (m->kind() == messages::Kind::Acknowledgment)
            // {
            //     _m_curator->acknowledge(m->id());
            // }
            auto handler = _m_factory->getHandler(m->payloadKind());
            return handler->handleMessage(_m_session, std::move(m));
        }
    };
}
