// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* mediator.hpp 

*/
#pragma once
#include <type_traits>
#include <stdexcept>

#include <indiemotion/_common.hpp>
#include <indiemotion/server/connection.hpp>
#include <indiemotion/session/session.hpp>
#include <indiemotion/messages/message.hpp>
#include <indiemotion/properties/session.hpp>

namespace indiemotion::session
{
    class SessionMediator
    {
        std::weak_ptr<Session> _m_session;

    public:
        SessionMediator(std::weak_ptr<Session> session) : _m_session(session) {}

        void handleMessage(messages::InitSessionMsg message)
        {   

            std::cout << "UID: " << message.uid << std::endl;

            if (auto current = _m_session.lock())
            {
                current->initialize(message.props);
            }
        }
    };

    class ConnectionMediator
    {
        std::weak_ptr<server::Connection> _m_connection;

        public:

            ConnectionMediator() = default;

            ConnectionMediator(std::weak_ptr<server::Connection> conn): _m_connection(conn) {}

            messages::UID ackInitialization(properties::SessionProperties props)
            {
                auto msg = messages::AckInitSessionMsg(props);
                if (auto conn = _m_connection.lock())
                {
                    conn->send(msg);
                }

                return msg.uid;
            }
    };

}