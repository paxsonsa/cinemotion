#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <fstream>
#include <thread>
#include <sstream>

#include <doctest.h>
#include <google/protobuf/util/message_differencer.h>
#include <google/protobuf/util/json_util.h>
#include <rapidjson/document.h>
#include <rapidjson/istreamwrapper.h>
#include <rapidjson/writer.h>
#include <rapidjson/stringbuffer.h>

// TODO move into 'testing' include folder
#include <configure.hpp>
#include <fixtures/playbook.hpp>
#include <indiemotion/net/message.hpp>
#include <indiemotion/net/dispatch.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/logging.hpp>

auto LOGGER = indiemotion::logging::getLogger("testing");

struct ResponseDispatcher : public indiemotion::NetMessageDispatcher {

    std::map<std::string, std::function<void(indiemotion::NetMessage &&)>> responseExpectations;

    void dispatch(indiemotion::NetMessage &&message) override {
        if (responseExpectations.count(message.header().responseid())) {
            responseExpectations[message.header().responseid()](std::move(message));
        }
    }
};

TEST_CASE("Execute Basic Start Up Integration")
{
//    auto delegate = std::make_shared<DummyDelegate>();
    auto ioContext = std::make_shared<asio::io_context>();
    auto work = asio::require(ioContext->get_executor(),
                              asio::execution::outstanding_work.tracked);
    std::thread thread([&] { ioContext->run(); });

    auto session = std::make_shared<indiemotion::SessionController>();
    auto dispatcher = std::make_shared<ResponseDispatcher>();
    auto bridge = indiemotion::SessionBridge(dispatcher, session);
    auto playbook_path = testing::getResourcePathFor("playbooks/basic_start_up.json");
    auto conf_path = testing::getResourcePathFor("configure/default.json");
    auto doc = testing::loadPlaybookDocument(playbook_path);

    bool failed = false;
    int count = 1;
    for (auto &v: doc.GetArray()) {

        std::cout << "Message [" << count << "] -----------------------------" << std::endl;
        auto message = testing::loadMessageObject(v);
        auto expect = testing::loadExpectObject(v);

        std::string m;
        testing::messageObjectIntoString(message, m);
        std::cout << "     message: " << m << std::endl;

        if (expect) {
            // Set Expected Response Header ID to match messages generated one.
            expect.value().mutable_header()->set_responseid(message.header().id());

            std::string e;
            testing::messageObjectIntoString(expect.value(), e);
            std::cout << "     expect: " << e << std::endl;
        } else {
            std::cout << "     expect: null" << std::endl;
        }

        if (expect) {
            auto expected = expect.value();
            dispatcher->responseExpectations[message.header().id()] =
                [&, id = count, expected = std::move(expected)](indiemotion::NetMessage &&message) {
                    google::protobuf::util::MessageDifferencer diff;
                    diff.IgnoreField(expected.descriptor()->FindFieldByName("header"));

                    // NOTE: Important this is called before Compare.
                    std::string report;
                    diff.ReportDifferencesToString(&report);

                    if (!diff.Compare(message, expected)) {
                        failed = true;
                        std::stringstream stream;
                        stream << "FAILED: Message [" << id << "] "
                               << "response does not match expected: \n";

                        std::string b1;
                        google::protobuf::util::MessageToJsonString(message, &b1);
                        stream << "+ Received: " << b1 << "\n";

                        std::string b2;
                        google::protobuf::util::MessageToJsonString(expected, &b2);
                        stream << "+ Expected: " << b2 << "\n";
                        stream << "----------------------------------------";
                        stream << report;
                        std::cerr << stream.str();
                    }
                };
        } else {
            dispatcher->responseExpectations[message.header().id()] =
                [&, id = count](indiemotion::NetMessage &&message) {
                    failed = true;
                    std::stringstream stream;
                    stream << "FAILED: Message [" << id << "] "
                           << "    did not expect message to be delivered\n";

                    std::string b;
                    google::protobuf::util::MessageToJsonString(message, &b);
                    stream << "    unexpected response: " << b << "\n";
                    stream << " DIFF ----------------------------------------\n";
                    std::cerr << stream.str() << "\n";

                };
        }

        ioContext->dispatch([&, message = std::move(message)]() mutable {
            std::string b;
            google::protobuf::util::JsonPrintOptions options;
            google::protobuf::util::MessageToJsonString(message, &b);
            try {
                bridge.processMessage(std::move(message));
            } catch (const std::exception &e) {
                LOGGER->info("Failed to Process Message: {}\n", e.what());
                failed = true;
                ioContext->stop();
            }
        });

        count += 1;
    }

    ioContext->post([&] {
        ioContext->stop();
    });
    thread.join();

    REQUIRE_FALSE(failed);
    REQUIRE(false);
}