#pragma once
#include <indiemotion/cameras/camera.hpp>
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>

namespace indiemotion::net
{
    struct GetCameraList : public NetPayload_T
    {
        NetPayloadType type() const
        {
            return NetPayloadType::GetCameraList;
        }
    };

    struct CameraList : public NetPayload_T
    {
        std::vector<cameras::Camera> cameras;

        CameraList(std::vector<cameras::Camera> cameras) : cameras(cameras) {}

        NetPayloadType type() const
        {
            return NetPayloadType::CameraList;
        }
    };

    struct SetCamera : public NetPayload_T
    {
        std::string cameraId;

        SetCamera(std::string id) : cameraId(id) {}

        NetPayloadType type() const
        {
            return NetPayloadType::SetCamera;
        }
    };

    struct CameraInfo : public NetPayload_T
    {
        std::optional<cameras::Camera> camera;

        CameraInfo(cameras::Camera camera) : camera(camera) {}

        NetPayloadType type() const
        {
            return NetPayloadType::CameraInfo;
        }
    };
} // namespace indiemotion::cameras
