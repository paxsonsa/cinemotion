#include <indiemotion/common.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/net/message.hpp>
#include <google/protobuf/util/json_util.h>

/*
Test Format
 [
    {
        send: {} // Message to Send
        expect: null | msg  // Wait for and test the expected response, when null it will not wait.
    },
 ]
 */

struct ServerMessageDispatcher: public indiemotion::NetMessageDispatcher
{
    std::shared_ptr<asio::io_context> ioContext;
    void dispatch(indiemotion::NetMessage &&message) override {

    }
};

struct ClientMessageDispatcher: public indiemotion::NetMessageDispatcher
{
    void dispatch(indiemotion::NetMessage &&message) override {

    }
};

int main()
{
//    auto ioContext = std::make_shared<asio::io_context>();
//    auto work = asio::require(ioContext->get_executor(),
//                             asio::execution::outstanding_work.tracked);
//
//    auto delegate = std::make_shared<DummyDelegate>();
//    auto session = std::make_shared<indiemotion::SessionController>();
//    auto dispatcher = std::make_shared<ServerMessageDispatcher>();
//    dispatcher->ioContext = ioContext;
//    auto bridge = indiemotion::SessionBridge(dispatcher, session);
//    bridge.start();

    // TODO load the test format and then process the JSON.
    // TODO Read Protobuf In the Dispatch a Message process

//    ioContext->run();

    indiemotion::NetMessage message;
    auto header = message.mutable_header();
    header->set_id("test");

    auto payload = message.mutable_motion_set_mode();
    payload->set_mode(indiemotion::netPayloadsV1::MotionMode::Live);

    std::string buffer;
    google::protobuf::util::JsonPrintOptions options;
    google::protobuf::util::MessageToJsonString(message, &buffer);

    std::cout << buffer << "\n";
    return 0;
}
