#pragma once
#include <indiemotion/common.hpp>

#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

namespace indiemotion::net
{
    /**
     * @brief Identifier for transport bodies
     *
     */
    using Identifier = std::string;

    /**
     * @brief Generate is new Identifier
     *
     * @return std::string
     */
    Identifier
    generateNewIdentifier()
    {
        boost::uuids::random_generator generator;
        boost::uuids::uuid uuid = generator();
        return boost::uuids::to_string(uuid);
    }

    /**
     * @brief A header for a transport object that.
     */
    class Header
    {
    private:
        Identifier _m_id;
        std::optional<Identifier> _m_responseToId;

    public:
        Header(Identifier id) : _m_id(id){};
        Header(Identifier id, Identifier responseId)
            : _m_id(id), _m_responseToId(responseId)
        {
        }

        Identifier
        id() const
        {
            return _m_id;
        }
        std::optional<Identifier>
        responseToId() const
        {
            return _m_responseToId;
        }
    };
}