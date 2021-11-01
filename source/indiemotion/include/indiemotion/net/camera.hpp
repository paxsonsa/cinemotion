#pragma once
#include <indiemotion/cameras/camera.hpp>
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>

namespace indiemotion::net
{
    struct GetCameraList : public net::Payload_T
    {
        PayloadType type() const
        {
            return PayloadType::GetCameraList;
        }
    };

    struct CameraList : public net::Payload_T
    {
        std::vector<cameras::Camera> cameras;

        CameraList(std::vector<cameras::Camera> cameras) : cameras(cameras) {}

        PayloadType type() const
        {
            return PayloadType::CameraList;
        }
    };

    struct SetCamera : public net::Payload_T
    {
        std::string cameraId;

        SetCamera(std::string id) : cameraId(id) {}

        PayloadType type() const
        {
            return PayloadType::SetCamera;
        }
    };

    struct CameraInfo : public net::Payload_T
    {
        std::optional<cameras::Camera> camera;

        CameraInfo(cameras::Camera camera) : camera(camera) {}

        PayloadType type() const
        {
            return PayloadType::CameraInfo;
        }
    };
} // namespace indiemotion::cameras
