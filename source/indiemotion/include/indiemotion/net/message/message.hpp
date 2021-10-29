#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message/header.hpp>
#include <indiemotion/net/message/payload.hpp>

namespace indiemotion::net
{
    /**
     * @brief A template for creating tranport containers.
     *
     */
    class Message
    {
    private:
        bool _m_requiresAck = false;
        std::shared_ptr<Header> _m_headerPtr;
        std::shared_ptr<Payload_T> _m_payloadPtr;

    public:
        Message(std::unique_ptr<Header> headerPtr,
                std::unique_ptr<Payload_T> payloadPtr)
            : _m_headerPtr(std::move(headerPtr)), _m_payloadPtr(std::move(payloadPtr))
        {
            assert(_m_headerPtr != nullptr && "message header cannot be nullptr");
            assert(_m_payloadPtr != nullptr && "message payload cannot be nullptr");
        }

        std::shared_ptr<Header> header() const
        {
            return _m_headerPtr;
        }

        std::shared_ptr<Payload_T> payload() const
        {
            return _m_payloadPtr;
        }

        void requiresAcknowledgement(bool s)
        {
            _m_requiresAck = s;
        }

        bool doesRequireAcknowledgement() const
        {
            return _m_requiresAck;
        }

        std::optional<Identifier> inResponseToId() const
        {
            return _m_headerPtr->responseToId();
        }

        bool isInReponseTo(Identifier id) const
        {
            if (_m_headerPtr->responseToId().has_value())
            {
                return _m_headerPtr->responseToId().value() == id;
            }
            return false;
        }

        Identifier id() const
        {
            return _m_headerPtr->id();
        }

        PayloadType payloadType() const
        {
            return _m_payloadPtr->type();
        }

        /**
         * @brief Return the payload at a cast to a particular type
         * 
         * @tparam T the object type to try and cast the payload too
         * @return std::shared_ptr<T> 
         */
        template <typename T>
        std::shared_ptr<T> payloadPtrAs() const
        {
            return std::dynamic_pointer_cast<T>(_m_payloadPtr);
        }
    };

    std::unique_ptr<Message> createMessage(Identifier inResponseToId,
                                           std::unique_ptr<Payload_T> payloadPtr)
    {
        auto id = generateNewIdentifier();
        auto headerPtr = std::make_unique<Header>(id, inResponseToId);
        auto containerPtr = std::make_unique<Message>(std::move(headerPtr), std::move(payloadPtr));

        return std::move(containerPtr);
    }

    std::unique_ptr<Message> createMessage(std::unique_ptr<Payload_T> payloadPtr)
    {
        auto id = generateNewIdentifier();
        auto headerPtr = std::make_unique<Header>(id);
        auto containerPtr = std::make_unique<Message>(std::move(headerPtr), std::move(payloadPtr));

        return std::move(containerPtr);
    }

} // namespace indiemotion::net