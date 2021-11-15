#pragma once
#include <fstream>

#include <google/protobuf/util/message_differencer.h>
#include <google/protobuf/util/json_util.h>
#include <rapidjson/document.h>
#include <rapidjson/istreamwrapper.h>
#include <rapidjson/writer.h>
#include <rapidjson/stringbuffer.h>

#include <configure.hpp>

#include <indiemotion/common.hpp>
#include <indiemotion/session.hpp>
#include <indiemotion/net/message.hpp>
#include "json_helpers.hpp"

namespace testing {

    void messageObjectIntoString(const indiemotion::NetMessage &message, std::string &s) {
        google::protobuf::util::JsonPrintOptions options;
        google::protobuf::util::MessageToJsonString(message, &s);
    }

    /**
     * Helper structure to store an individual item in the playbook
     *
     */
    struct PlaybookItem {
        /// An ID to identify the item
        int id;

        /// The message to dispatch to the server
        indiemotion::NetMessage message;

        /// An optional expected result from the server
        std::optional<indiemotion::NetMessage> expected;

        /// An optional error to expect as a response from the dispatched message.
        std::optional<indiemotion::NetMessage> error;
    };

    /**
     * Represents a playbook of message/responses to be dispatched and verified.
     */
    struct Playbook {
        std::vector<PlaybookItem> items{};

        /**
         * Construct a playbook by path
         *
         * This will load up the JSON file at the given path and construct the
         * playbook items found in the file.
         *
         * @param playbook_path
         */
        Playbook(std::string playbook_path) {
            auto doc = loadJSONDocument(playbook_path);
            int cur_id = 1;
            for (auto &v: doc.GetArray()) {
                PlaybookItem item;
                item.id = cur_id;
                cur_id += 1;
                item.message = std::move(loadMessageObject(v));
                item.expected = std::move(loadExpectObject(v));

                if (item.expected) {
                    // Set Expected Response Header ID to match messages generated one.
                    item.expected.value().mutable_header()->set_responseid(item.message.header().id());
                }
                items.push_back(item);
            }
        }
    };

    /**
     * A helper class to run a message playbook and validate their
     * responses. This is useful for quickly testing the full stack of
     * a session delegate and bridge.
     *
     */
    class PlaybookRunner {
        /**
         * A Dummy Dispatcher used for invoking the testing handlers that the
         * playbook defines.
         *
         * This should not be directly instanced
         *
         */
        struct DummyDispatcher : public indiemotion::NetMessageDispatcher {

            std::map<std::string, std::function<void(indiemotion::NetMessage &&)>> responseExpectations;
            void dispatch(indiemotion::NetMessage &&message) override {
                if (responseExpectations.count(message.header().responseid())) {
                    responseExpectations[message.header().responseid()](std::move(message));
                }
            }
        };

        std::shared_ptr<DummyDispatcher> dispatcher;
        std::shared_ptr<asio::io_context> io_context;
        bool failed = false;

    public:
        Playbook playbook;
        std::shared_ptr<indiemotion::SessionBridge> bridge;

        /**
         * Construct a new PlaybookRunner with the given playbook.
         * @param playbook
         */
        PlaybookRunner(Playbook &&playbook) : playbook(std::move(playbook)) {
            dispatcher = std::make_shared<DummyDispatcher>();
            auto session = std::make_shared<indiemotion::SessionController>();
            bridge = std::make_shared<indiemotion::SessionBridge>(dispatcher, std::move(session));

            io_context = std::make_shared<asio::io_context>();

        }

        /**
         * Initialize the runner to use the delegate as the session controller's delegate during runtime.
         * @param delegate
         */
        void initializeWithDelegate(std::shared_ptr<indiemotion::SessionControllerDelegate> delegate) {
            auto session = std::make_shared<indiemotion::SessionController>(delegate);
            bridge = std::make_shared<indiemotion::SessionBridge>(dispatcher, std::move(session));
        }

        /**
         * Run the playbook
         *
         * Each Item in the playbook is dispatched through the bridge and then
         * the expected result is verified for each item.
         *
         */
        void run() {
            // Create indefinite work so the context does not shut down.
            auto work = asio::require(io_context->get_executor(),
                                      asio::execution::outstanding_work.tracked);
            std::thread thread([&] { io_context->run(); });
            for (auto &item: playbook.items) {
                print_item(item);
                run_item(item);
            }
            io_context->post([&] {
                io_context->stop();
            });
            thread.join();
            REQUIRE_FALSE(failed);
        }

    private:
        void print_item(const PlaybookItem &item) {
            std::cout << "Message [" << item.id << "] -----------------------------" << std::endl;
            std::string m;
            testing::messageObjectIntoString(item.message, m);
            std::cout << "     message: " << m << std::endl;

            if (item.expected) {
                std::string e;
                testing::messageObjectIntoString(item.expected.value(), e);
                std::cout << "     expect: " << e << std::endl;
            } else {
                std::cout << "     expect: null" << std::endl;
            }
        }

        void run_item(const PlaybookItem &item) {
            if (item.expected) {
                auto expected = item.expected.value();
                dispatcher->responseExpectations[item.message.header().id()] =
                    [&, id = item.id, expected = std::move(expected)](indiemotion::NetMessage &&message) {
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
                dispatcher->responseExpectations[item.message.header().id()] =
                    [&, id = item.id](indiemotion::NetMessage &&message) {
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

            io_context->dispatch([&, message = std::move(item.message)]() mutable {
                std::string b;
                google::protobuf::util::JsonPrintOptions options;
                google::protobuf::util::MessageToJsonString(message, &b);
                try {
                    bridge->processMessage(std::move(message));
                } catch (const std::exception &e) {
                    std::cerr << "Failed to Process Message: " << e.what() << "\n";
                    failed = true;
                    io_context->stop();
                }
            });
        }
    };
}