//
// Created by Andrew Paxson on 2021-11-09.
//
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/session/properties.hpp>
#include <indiemotion/session/server_info.hpp>

namespace indiemotion
{
    struct NetSessionStart: public NetPayload_T
    {
        SessionServerInfo serverInfo;

        NetSessionStart(SessionServerInfo i): serverInfo(i) {}

        NetPayloadType type() const
        {
            return NetPayloadType::SessionStart;
        }
    };

    struct NetSessionActivate: public NetPayload_T
    {
        SessionProperties sessionProperties;

        NetSessionActivate(SessionProperties p): sessionProperties(p) {}

        NetPayloadType type() const
        {
            return NetPayloadType::SessionActivate;
        }
    };
}