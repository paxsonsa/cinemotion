#pragma once
#include <chrono>
#include <type_traits>

#include <indiemotion/_common.hpp>

namespace indiemotion::messages
{

    using UID = uint32_t;

    enum class Kind
    {
        Invalid = 0,
        Ack = 1,

        InitSession = 100,
    };

    std::map<Kind, std::string> KindNameMappings {
        {Kind::Invalid, "Invalid"},
        {Kind::InitSession, "Init"},
        {Kind::Ack, "Ack"},
    };

    class Message
    {   
        private:
            UID _m_uid;

        public:
            Message()
            {   
                using namespace std::chrono;
                _m_uid = duration_cast<milliseconds>(
                    system_clock::now().time_since_epoch()
                ).count();
            }

            virtual ~Message() {}

            /**
             * @brief Get the Id object
             * 
             * @return UID 
             */
            UID getId()
            {
                return _m_uid;
            }

            /**
             * @brief Get the Kind object
             * 
             * @return Kind 
             */
            virtual Kind getKind() = 0;

            /**
             * @brief Does this message require acknowledgment messagges
             * 
             * @return true 
             * @return false 
             */
            virtual bool needsAcknowledgment() = 0;
    };
}
