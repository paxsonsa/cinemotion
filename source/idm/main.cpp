#include <indiemotion/messages.hpp>
#include <indiemotion/protobuf.hpp>
#include <indiemotion/server.hpp>

#include <fmt/core.h>
#include <iostream>
#include <memory>

#include <google/protobuf/arena.h>
#include <google/protobuf/util/json_util.h>

int main()
{
    // auto options = std::make_unique<indiemotion::server::Options>();
    // auto server = std::make_unique<indiemotion::server::Server>(std::move(options));

    // fmt::print("Starting Server\n");
    // server->start();
    // fmt::print("Server Stopped.");
    GOOGLE_PROTOBUF_VERIFY_VERSION;

    indiemotion::protobuf::messages::ClientMessage message;
    auto headerPtr = message.mutable_header();
    headerPtr->set_id("someid");
    headerPtr->set_responseid("someOtherId");

    auto acknowledge = message.mutable_acknowledge();
    acknowledge->set_ok(true);
    acknowledge->set_message("hello world");

    auto wrapper = indiemotion::messages::wrappers::Factory::create(message);

    std::cout << "Wrapper Payload Kind: " << indiemotion::messages::kindToStr(wrapper->payloadKind()) << "\n";

    std::string json_message;
    auto opt = google::protobuf::util::JsonPrintOptions();
    // opt.add_whitespace = true;
    google::protobuf::util::MessageToJsonString(message, &json_message, opt);
    std::cout << json_message << std::endl;

    return 0;
}
