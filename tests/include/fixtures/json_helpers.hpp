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

namespace testing {
    /**
         * Load the JSON Document.
         * @param path
         * @return
         */
    rapidjson::Document loadJSONDocument(std::string path) {
        std::ifstream ifs(path);
        rapidjson::IStreamWrapper isw(ifs);
        rapidjson::Document doc;
        doc.ParseStream(isw);
        return std::move(doc);
    }
    /**
         * Load a 'description' item from the JSON playbook document
         * @param item An individual array item from the JSON playbook to extract the 'description' from.
         * @return
         */
    indiemotion::Message loadMessageObject(const rapidjson::Value &item) {
        rapidjson::StringBuffer buffer;
        rapidjson::Writer<rapidjson::StringBuffer> writer(buffer);
        item["description"].Accept(writer);

        indiemotion::Message message;
        google::protobuf::util::JsonStringToMessage(buffer.GetString(), &message);
        message.mutable_header()->set_id(indiemotion::net_generate_new_identifier_string());

        return std::move(message);
    }
    /**
         * Load an 'expected' item from the playbook JSON document.
         * @param item An individual array item from the JSON playbook.
         * @return
         */
    std::optional<indiemotion::Message> loadExpectObject(const rapidjson::Value &item) {
        if (!item.HasMember("expected")) {
            std::cerr << "malformed playbook, missing 'expected' item in test item" << std::endl;
            throw std::runtime_error("malformed playbook, missing 'expected' item in test item");
        }

        if (item["expected"].IsNull()) {
            return {};
        }

        auto expected = item["expected"].GetObject();
        rapidjson::StringBuffer b;
        rapidjson::Writer<rapidjson::StringBuffer> w(b);
        item["expected"].Accept(w);
        indiemotion::Message e;
        google::protobuf::util::JsonStringToMessage(b.GetString(), &e);

        return std::move(e);
    }
}