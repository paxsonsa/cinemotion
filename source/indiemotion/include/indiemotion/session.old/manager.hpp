// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* manager.hpp

*/
#pragma once
#include <spdlog/spdlog.h>

#include <indiemotion/common.hpp>
#include <indiemotion/errors.hpp>
#include <indiemotion/messages.hpp>
#include <indiemotion/responses.hpp>
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
        std::unique_ptr<responses::base::Response> initialize()
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
            auto payload = std::make_unique<responses::session::initialize::Payload>(properties);

            auto responsePtr = responses::base::createResponse(std::move(payload));

            // Register a ack callback with the curator
            _m_curator->queue(responsePtr->header()->id(), [&]()
                              { _m_session->activate(); });

            return responsePtr;
        }

        /**
         * @brief Return
         *
         * @param m
         * @return std::optional<std::shared_ptr<messages::Handler>>
         */
        std::optional<std::unique_ptr<responses::base::Response>> processMessage(std::unique_ptr<messages::base::Message> m)
        {
            if (m->payloadKind() == messages::Kind::Acknowledgment)
            {
                if (!m->inResponseToId().has_value())
                {
                    // TODO back to client
                    spdlog::error("acknowledgement '{}' does not have a 'inResponseTo' ID", m->header()->id());
                    return {};
                }

                _m_curator->acknowledge(m->inResponseToId().value());
                return {};
            }
            auto handler = _m_factory->getHandler(m->payloadKind());
            return handler->handleMessage(_m_session, std::move(m));
        }
    };
}
