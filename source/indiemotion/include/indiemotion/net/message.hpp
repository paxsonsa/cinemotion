#pragma once

namespace indiemotion::net
{
    /**
     * @brief PayloadType stores the type of payload that a message contains so
     *        we can perform operations based on that content.
     *
     */
    enum class PayloadType : std::int32_t
    {
        // ---------------------------------------------------------
        // General Payload Types
        Unknown,
        Error,
        Acknowledge,

        // ---------------------------------------------------------
        // Session Payload Types
        SessionInitilization,
        SessionShutdown,

        // ---------------------------------------------------------
        // Camera Payload Types
        GetCameraList,
        CameraList,
        SetCamera,
        CameraInfo,

        // ---------------------------------------------------------
        // Motion Payload Types
        MotionGetMode,
        MotionSetMode,
        MotionActiveMode,
        MotionUpdateXForm,
    };

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
        std::optional<Identifier> _m_responseToId;
        Identifier _m_id;

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

    /**
     * @brief The body of a message transport, this should be subclassed
     *
     */
    class Payload_T
    {
      public:
        Payload_T() = default;
        virtual ~Payload_T() {}

        /**
         * @brief Return the kind of body
         *
         * @return Kind
         */
        virtual PayloadType type() const = 0;
    };

    /**
     * @brief A message is the main transportable type through
     *        the network API.
     *
     */
    class Message
    {
      private:
        bool _m_requiresAck = false;
        Identifier _m_id;
        std::optional<Identifier> _m_responseToId;
        std::shared_ptr<Payload_T> _m_payloadPtr;
        std::shared_ptr<spdlog::logger> _logger;

        void init()
        {
            _logger = logging::getLogger("com.indiemotion.net.message");
            assert(_m_payloadPtr != nullptr && "message payload cannot be nullptr");
        }

      public:
        Message(Idenitfier id,
                std::unique_ptr<Payload_T> payloadPtr)
            : _m_id(id), _m_payloadPtr(std::move(payloadPtr))
        {
            init();
        }

        Message(Identifier id,
                Identifier responseId,
                std::unique_ptr<Payload_T> payloadPtr)
            : _m_id(id), _m_responseToId(responseId), _m_payloadPtr(std::move(payloadPtr))
        {
            init();
        }

        [[nodiscard]] std::shared_ptr<Payload_T> payload() const
        {
            return _m_payloadPtr;
        }

        void requiresAcknowledgement(bool s)
        {
            _m_requiresAck = s;
        }

        [[nodiscard]] bool doesRequireAcknowledgement() const
        {
            return _m_requiresAck;
        }

        [[nodiscard]] std::optional<Identifier> inResponseToId() const
        {
            return _m_responseToId;
        }

        [[nodiscard]] bool isInResponseTo(Identifier id) const
        {
            if (_m_responseToId)
            {
                return _m_responseToId == id;
            }
            return false;
        }

        [[nodiscard]] Identifier id() const
        {
            return _m_id;
        }

        [[nodiscard]] PayloadType payloadType() const
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
            _logger->trace("casting message payload as {}", typeid(T).name());
            return std::dynamic_pointer_cast<T>(_m_payloadPtr);
        }
    };

    /**
     * Create a new message with a given `Identifier`
     * @param id An Identifier to use for the message id
     * @param payloadPtr A unique ptr to the payload the message contains
     * @return
     */
    std::unique_ptr<Message> createMessageWithId(const Identifier id,
                                                 std::unique_ptr<Payload_T> payloadPtr)
    {
        auto containerPtr = std::make_unique<Message>(std::move(id), std::move(payloadPtr));
        return std::move(containerPtr);
    }


    std::unique_ptr<Message> createMessage(const Identifier id,
                                           const Identifier inResponseToId,
                                           std::unique_ptr<Payload_T> payloadPtr)
    {
        auto containerPtr = std::make_unique<Message>(std::move(id),
                                                      std::move(inResponseToId),
                                                      std::move(payloadPtr));
        return std::move(containerPtr);
    }

    std::unique_ptr<Message> createMessage(const Identifier &inResponseToId,
                                           std::unique_ptr<Payload_T> payloadPtr)
    {
        auto id = generateNewIdentifier();
        auto containerPtr = std::make_unique<Message>(std::move(id),
                                                      std::move(inResponseToId),
                                                      std::move(payloadPtr));
        return std::move(containerPtr);
    }

    std::unique_ptr<Message> createMessage(std::unique_ptr<Payload_T> payloadPtr)
    {
        auto id = generateNewIdentifier();
        auto containerPtr = std::make_unique<Message>(std::move(id), std::move(payloadPtr));
        return std::move(containerPtr);
    }

}