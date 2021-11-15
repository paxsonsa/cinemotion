#pragma once
#include <indiemotion/session.hpp> //  indiemotion::SessionControllerDelegate
#include <fixtures/json_helpers.hpp> // rapidjson::Document, loadJSONDocument

namespace testing {
    struct JSONConfiguredDelegate : indiemotion::SessionControllerDelegate {
        rapidjson::Document document;

        JSONConfiguredDelegate(std::string json_path) {
            document = loadJSONDocument(json_path);
        }

        std::vector<indiemotion::cameras::Camera> getAvailableCameras() override {
            if (document.HasMember("getAvailableCameras")) {
                std::vector<indiemotion::cameras::Camera> cameras {};
                for (auto &item: document["getAvailableCameras"].GetArray()) {
                    auto camera = indiemotion::cameras::Camera(
                        item["id"].GetString()
                    );
                    cameras.push_back(camera);
                }
                return cameras;
            }
            return std::vector<indiemotion::cameras::Camera>();
        }
    };
}