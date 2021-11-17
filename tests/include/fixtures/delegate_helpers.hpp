#pragma once
#include <indiemotion/session.hpp> //  indiemotion::SessionControllerDelegate, indiemotion::Camera
#include <fixtures/json_helpers.hpp> // rapidjson::Document, loadJSONDocument

namespace testing {
    struct JSONConfiguredDelegate : indiemotion::SessionControllerDelegate {
        rapidjson::Document document;

        JSONConfiguredDelegate(std::string json_path) {
            document = loadJSONDocument(json_path);
        }

        std::vector<indiemotion::Camera> get_available_cameras() override {
            if (document.HasMember("get_available_cameras")) {
                std::vector<indiemotion::Camera> cameras{};
                for (auto &item: document["get_available_cameras"].GetArray()) {
                    auto camera = indiemotion::Camera(
                        item["id"].GetString()
                    );
                    cameras.push_back(camera);
                }
                return cameras;
            }
            return std::vector<indiemotion::Camera>();
        }

        std::optional<indiemotion::Camera> get_camera_by_name(std::string id) override {
            if (document.HasMember("get_camera_by_name")) {
                auto cid = document["get_camera_by_name"].GetObject()["id"].GetString();
                indiemotion::Camera camera(cid);

                if (cid == id) {
                    return camera;
                }
            }
            return {};
        }
    };
}