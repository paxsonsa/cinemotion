# IndieMotion Server 
A framework for capturing motion from an external device.

IndieMotion is framework that can be implemented into digital content creation (DCC) software
to allow for external devices to transmit/record camera motion data into the application.

# Build and Usage
Indiemotion is conformant with the [VFX 2022 Reference](https://vfxplatform.com) Platform. 

| Library | Version | Included? |
| ------------ | ------------- | ----------|
boost | 1.76.0 | No
protobuf3-cpp | 3.17.3 | No
spdlog | 1.9.2 | yes |
fmt | 8.0.1| yes |
doctest | 2.4.6 | yes|

Build the tests and the debug server
```bash
mkdir build
cd build
cmake ..
cmake --build . --target all
ctest
```

## Basic Server Implementation
IndieMotion works by establishing a server that listens for incoming connections
and establishes a connection and session. As a DCC Implementor, your job is to provide the glue
layer for manipulating your DCCs environment by providing an implementation of a few delegates. Each delegate is 
responsible for receiving updates to certain parts of the context data.

The `indiemotion::Context` is a composite value-type of other sub-context structures:
- `SceneContext` contains information about the actual 3D scene and objects, specifically the cameras.
- `SessionContext` contains information about the current session state (e.g. the name, is it initialized, etc.)
- `MotionContext` contains the information about the current transform and motion capture status.

An example of a simple implementation can be found in the `source/idmserver/main.cpp`

A basic implementation does a few things:
1. Implement the delegates for each context.
2. Define the options for the server.
3. Build an Instance of `indiemotion::Server`
4. Start the Server in a new thread.

Below is a naive implementation
```cpp
#include <indiemotion/server.hpp>

using idm = indiemotion;

struct DebugDelegate: public idm::SessionDelegate, idm::SceneDelegate, idm::MotionDelegate {
    ...
};

// Within some function/class
auto delegate = std::make_shared<DebugDelegate>();
DelegateInfo delegate_info;
delegate_info.session = delegate;
delegate_info.scene = delegate;
delegate_info.motion = delegate;

Options server_options;
server_options.address = "0.0.0.0";
server_options.port = 7766;
server_options.delegate_info = delegate_info;

server_options.on_connect = [&]() {};
server_options.on_disconnect = [&]() {};

auto server = Server(server_options);
std::thread thread{[&server]() {
    server.start();
}};

thread.join();
return 0;
...
```

## Subprojects
| Path | Description|
|-----|----|
| `source/idmserver` | A test server for building basic functionality for InputDevices. |
| `source/indiemotion` | A C++ Framework for building IndieMotion compliant applications |
| `source/indiemotiondb` | The protobuf generated source code for protocol messages |
