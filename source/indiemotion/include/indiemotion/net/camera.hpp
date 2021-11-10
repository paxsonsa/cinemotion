#pragma once
#include <indiemotion/cameras/camera.hpp>
#include <indiemotion/common.hpp>
#include <indiemotion/net/message.hpp>

namespace indiemotion
{
    struct NetGetCameraList : public NetPayload_T
    {
        NetPayloadType type() const
        {
            return NetPayloadType::GetCameraList;
        }
    };

    struct NetCameraList : public NetPayload_T
    {
        std::vector<cameras::Camera> cameras;

        NetCameraList(std::vector<cameras::Camera> cameras) : cameras(cameras) {}

        NetPayloadType type() const
        {
            return NetPayloadType::CameraList;
        }
    };

    struct NetSetActiveCamera : public NetPayload_T
    {
        std::string cameraId;

        NetSetActiveCamera(std::string id) : cameraId(id) {}

        NetPayloadType type() const
        {
            return NetPayloadType::SetActiveCamera;
        }
    };

    struct NetActiveCameraInfo : public NetPayload_T
    {
        std::optional<cameras::Camera> camera;

        NetActiveCameraInfo(cameras::Camera camera) : camera(camera) {}

        NetPayloadType type() const
        {
            return NetPayloadType::ActiveCameraInfo;
        }
    };
} // namespace indiemotion::cameras
