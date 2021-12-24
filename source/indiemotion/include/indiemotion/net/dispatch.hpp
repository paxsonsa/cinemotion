#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>

namespace indiemotion {
    struct NetMessageDispatcher {
        virtual void dispatch(NetMessage &&message) = 0;
    };
}