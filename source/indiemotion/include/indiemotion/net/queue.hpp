//
// Created by Andrew Paxson on 2021-11-06.
//
#pragma once
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>
#include <boost/lockfree/spsc_queue.hpp>

namespace indiemotion::net
{
  using MessageQueue = boost::lockfree::spsc_queue<std::shared_ptr<Message>, boost::lockfree::capacity<1024>>;
}