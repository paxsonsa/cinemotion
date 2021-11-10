#pragma once

namespace indiemotion::net {
    /**
     * @brief PayloadType stores the type of payload that a message contains so
     *        we can perform operations based on that content.
     *
     */
    enum class PayloadType : std::int32_t {
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
    Identifier generateNewIdentifier() {
        boost::uuids::random_generator generator;
        boost::uuids::uuid uuid = generator();
        return boost::uuids::to_string(uuid);
    }

    /**
     * @brief The body of a message transport, this should be subclassed
     *
     */
    class Payload_T {
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
    class Message {
    private:
        bool _m_requiresAck = false;
        Identifier _m_id;
        std::optional<Identifier> _m_responseToId;
        std::shared_ptr<Payload_T> _m_payloadPtr;
        std::shared_ptr<spdlog::logger> _logger;

        void init() {
            _logger = logging::getLogger("com.indiemotion.net.message");
            assert(_m_payloadPtr != nullptr && "message payload cannot be nullptr");
        }

    public:
        /**
         * Construct the message with a known id and payload.
         * @param id Identifier for the message.
         * @param payloadPtr A std::unique_ptr to the payload type.
         */
        Message(Idenitfier id, std::unique_ptr<Payload_T> payloadPtr)
            : _m_id(id), _m_payloadPtr(std::move(payloadPtr)) {
            init();
        }

        /**
         * Construct the message with a know id, responseId, and payload.
         * @param id An identifier for the message.
         * @param responseId A response Id the message is in response too.
         * @param payloadPtr A unique_ptr to the payload of the message.
         */
        Message(Identifier id, Identifier responseId,
                std::unique_ptr<Payload_T> payloadPtr)
            : _m_id(id), _m_responseToId(responseId),
              _m_payloadPtr(std::move(payloadPtr)) {
            init();
        }

        /**
         * Return a shared ptr to the payload.
         * @return the message payload uncast
         */
        [[nodiscard]] std::shared_ptr<Payload_T> payload() const {
            return _m_payloadPtr;
        }

        /**
         * Set whether the message requires an acknowledgement
         * @param s
         */
        void requiresAcknowledgement(bool s) { _m_requiresAck = s; }

        /**
         * Does this message require and acknowledgement that it has been processed?
         * @return whether the message requires an ACK.
         */
        [[nodiscard]] bool doesRequireAcknowledgement() const {
            return _m_requiresAck;
        }

        /**
         * Return a potential id that the message is in response to.
         * @return An identifier for the response.
         */
        [[nodiscard]] std::optional<Identifier> inResponseToId() const {
            return _m_responseToId;
        }

        /**
         * The message unique identifier.
         * @return An Identifier for the message
         */
        [[nodiscard]] Identifier id() const { return _m_id; }

        /**
         * Get the payload type this message contains (forwards request to Payload
         * object)
         * @return The PayloadType this message is containing
         */
        [[nodiscard]] PayloadType payloadType() const {
            return _m_payloadPtr->type();
        }

        /**
         * @brief Return the payload at a cast to a particular type
         *
         * @tparam T the object type to try and cast the payload too
         * @return std::shared_ptr<T>
         */
        template<typename T> std::shared_ptr<T> payloadPtrAs() const {
            _logger->trace("casting message payload as {}", typeid(T).name());
            return std::dynamic_pointer_cast<T>(_m_payloadPtr);
        }
    };

    /**
     * Make a new message and generate the ID.
     * @param payloadPtr A unique ptr to the payload the message contains.
     * @return unique pointer to a new message
     */
    std::unique_ptr<Message> createMessage(std::unique_ptr<Payload_T> payloadPtr) {
        auto id = generateNewIdentifier();
        auto containerPtr =
            std::make_unique<Message>(std::move(id), std::move(payloadPtr));
        return std::move(containerPtr);
    }

    /**
     * Make a new message with a given ID and Response ID
     * @param id An Identifier to use for the message ID.
     * @param inResponseToId An Identifier for the response ID.
     * @param payloadPtr A unique ptr to the payload the message contains.
     * @return unique pointer to a new message
     */
    std::unique_ptr<Message> makeMessageWithIdAndResponseId(const Identifier id,
                                                            const Identifier inResponseToId,
                                                            std::unique_ptr<Payload_T> payloadPtr) {
        auto containerPtr = std::make_unique<Message>(std::move(id),
                                                      std::move(inResponseToId),
                                                      std::move(payloadPtr));
        return std::move(containerPtr);
    }

    /**
     * Make a new message with a given ID
     * @param id An Identifier to use for the message id
     * @param payloadPtr A unique ptr to the payload the message contains
     * @return unique pointer to a new message
     */
    std::unique_ptr<Message> makeMessageWithId(const Identifier id,
                                               std::unique_ptr<Payload_T> payloadPtr) {
        auto containerPtr =
            std::make_unique<Message>(std::move(id), std::move(payloadPtr));
        return std::move(containerPtr);
    }

    /**
     * Make a new message with a given Response ID ( and generate ID)
     * @param inResponseToId An Identifier for the response ID.
     * @param payloadPtr A unique ptr to the payload the message contains.
     * @return unique pointer to a new message
     */
    std::unique_ptr<Message> makeMessageWithResponseID(const Identifier &inResponseToId,
                                                       std::unique_ptr<Payload_T> payloadPtr) {
        auto id = generateNewIdentifier();
        auto containerPtr = std::make_unique<Message>(std::move(id),
                                                      std::move(inResponseToId),
                                                      std::move(payloadPtr));
        return std::move(containerPtr);
    }

} // namespace indiemotion::net