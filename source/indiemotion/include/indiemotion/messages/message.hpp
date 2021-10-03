#pragma once
#include <chrono>
#include <type_traits>

#include <indiemotion/_common.hpp>

namespace indiemotion::messages
{

    typedef uint32_t UID;

    enum class Kind
    {
        Invalid = -1,
        Ack = 0,

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

            virtual Kind get_kind()
            {
                return Kind::Invalid;
            };

            UID get_uid()
            {
                return _m_uid;
            }

    };
}
