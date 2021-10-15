// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* header.hpp */
#pragma once
#include <indiemotion/common.hpp>

#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

namespace indiemotion::transport
{
    class Header
    {
    private:
        std::string _m_id;
        std::optional<std::string> _m_inResponseToId;

    public:
        Header(std::string id) : _m_id(id) {}
        Header(std::string id, std::string responseId) : _m_id(id), _m_inResponseToId(responseId) {}

        std::string id()
        {
            return _m_id;
        }

        std::optional<std::string> inResponseToId()
        {
            return _m_inResponseToId;
        }
    };

    std::string generateNewId()
    {
        boost::uuids::random_generator generator;
        boost::uuids::uuid uuid = generator();
        return boost::uuids::to_string(uuid);
    }
}
