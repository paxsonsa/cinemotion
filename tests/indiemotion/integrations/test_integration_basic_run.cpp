#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include <configure.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/session.hpp>

#include <google/protobuf/util/json_util.h>
#include <rapidjson/document.h>
#include <rapidjson/istreamwrapper.h>
#include <rapidjson/writer.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/error/en.h>

#include <fstream>
#include <thread>

struct ResponseDispatcher : public indiemotion::NetMessageDispatcher {
    void dispatch(indiemotion::NetMessage &&message) override {

    }
};

TEST_CASE("Execute Basic Start Up Integration")
{
//    auto delegate = std::make_shared<DummyDelegate>();
    auto logger = indiemotion::logging::getLogger("com.indiemotion.testing");
    auto ioContext = std::make_shared<asio::io_context>();
    auto work = asio::require(ioContext->get_executor(),
                              asio::execution::outstanding_work.tracked);
    std::thread thread([&] {ioContext->run();});

    auto session = std::make_shared<indiemotion::SessionController>();
    session->setStatus(indiemotion::SessionStatus::Activated);

    auto dispatcher = std::make_shared<ResponseDispatcher>();
    auto bridge = indiemotion::SessionBridge(dispatcher, session);

    auto playbook_path = testing::getResourcePathFor("playbooks/basic_start_up.json");
    auto conf_path = testing::getResourcePathFor("configure/default.json");

    std::cout << playbook_path << "\n";
    std::ifstream ifs(playbook_path);
    rapidjson::IStreamWrapper isw(ifs);
    rapidjson::Document doc;
    doc.ParseStream(isw);

    bool failed = false;

    for (auto &v: doc.GetArray()) {
        if (v.HasMember("message") && !v["message"].IsNull()) {
            rapidjson::StringBuffer buffer;
            rapidjson::Writer<rapidjson::StringBuffer> writer(buffer);
            v["message"].Accept(writer);

            indiemotion::NetMessage message;
            google::protobuf::util::JsonStringToMessage(buffer.GetString(), &message);

            // TODO Post Work to MessageDispatcher
            ioContext->dispatch([&, message = std::move(message)]() mutable {
                std::string b;
                google::protobuf::util::JsonPrintOptions options;
                google::protobuf::util::MessageToJsonString(message, &b);
                logger->info("Processing Message: {}\n", b);
                try {
                    bridge.processMessage(std::move(message));
                } catch (const std::exception& e) {
                    logger->error("Failed to Process Message: {}\n", e.what());
                    failed = true;
                    ioContext->stop();
                }
            });
        }
//        if (v.HasMember("expected") && !v["expected"].IsNull()) {
//            auto expected = v["expected"].GetObject();
//            rapidjson::StringBuffer buffer;
//            rapidjson::Writer<rapidjson::StringBuffer> writer(buffer);
//            v["expected"].Accept(writer);
//
//            std::cout << "expected: " << buffer.GetString() << "\n";
//        }
    }
    ioContext->post([&]{
        ioContext->stop();
    });
    thread.join();

    REQUIRE_FALSE(failed);
    REQUIRE(false);
}