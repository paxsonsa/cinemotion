#pragma once 

namespace indiemotion::messages {

struct Message;

typedef std::function<void(messages::Message)> MessageHandler;

struct Message {};

}
