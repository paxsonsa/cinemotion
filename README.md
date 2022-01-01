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
layer for manipulating your DCCs environment by providing an implementation of `indiemotion::SessionControllerDelegate`.

An example of a simple implementation can be found in the `source/idmserver/main.cpp`

A basic implementation does a few things:
1. Implement the `indiemotion::SessionControllerDelegate`
2. Build an Instance of `indiemotion::SessionServer`
3. Start the Server in a new thread and pass in a `on_start` callback to attach your delegate to the given `SessionController`

Below is a naive implementation
```cpp
#include <indiemotion/session.hpp>
#include <indiemotion/server.hpp>

using idm = indiemotion;

struct DebugDelegate: public idm::SessionControllerDelegate {
    ...
};

// Within some function/class
    ServerOptions server_options;
    server_options.address = "0.0.0.0";
    server_options.port = 7766;
    
    auto server = SessionServer(server_options);
    
    // Start Server in thread
    std::thread thread{[&server]() {
        server.start([](std::shared_ptr<SessionController> controller) {
            // Create the SessionControllerDelegate
            auto delegate = std::make_shared<DebugDelegate>();
            
            // Pass Ownership to the controller.
            controller->set_delegate(std::move(delegate));
        });
    }};
...
```

## Subprojects
| Path | Description|
|-----|----|
| `source/idmserver` | A test server for building basic functionality for InputDevices. |
| `source/indiemotion` | A C++ Framework for building IndieMotion compliant applications |
| `source/indiemotiondb` | The protobuf generated source code for protocol messages |
