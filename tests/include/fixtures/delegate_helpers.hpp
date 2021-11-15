#pragma once
#include <indiemotion/session.hpp> //  indiemotion::SessionControllerDelegate, indiemotion::Camera
#include <fixtures/json_helpers.hpp> // rapidjson::Document, loadJSONDocument

namespace testing {
    struct JSONConfiguredDelegate : indiemotion::SessionControllerDelegate {
        rapidjson::Document document;

        JSONConfiguredDelegate(std::string json_path) {
            document = loadJSONDocument(json_path);
        }

        std::vector<indiemotion::Camera> getAvailableCameras() override {
            if (document.HasMember("getAvailableCameras")) {
                std::vector<indiemotion::Camera> cameras{};
                for (auto &item: document["getAvailableCameras"].GetArray()) {
                    auto camera = indiemotion::Camera(
                        item["id"].GetString()
                    );
                    cameras.push_back(camera);
                }
                return cameras;
            }
            return std::vector<indiemotion::Camera>();
        }

        std::optional<indiemotion::Camera> getCameraById(std::string id) override {
            if (document.HasMember("getCameraById")) {
                auto cid = document["getCameraById"].GetObject()["id"].GetString();
                indiemotion::Camera camera(cid);

                if (cid == id) {
                    return camera;
                }
            }
            return {};
        }
    };
}