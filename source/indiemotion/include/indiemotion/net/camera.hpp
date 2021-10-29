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
} // namespace indiemotion::cameras
