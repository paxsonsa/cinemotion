#pragma once

namespace indiemotion::device {
struct DeviceProperties {

    static DeviceProperties thisDeviceProperties()
    {
        return DeviceProperties();
    }

};
struct ClientProperties {};
}
