# TDOD Notes
- Python API
  - How will we handle the threaded server and callbacks?
  - https://pythonextensionpatterns.readthedocs.io/en/latest/thread_safety.html
- Handle Error Messages
- InputDevice
  - Interaction from Controller to Service/InputDevice
- Reset/Set Origin
- Reset session
- Video Streaming
  - Road to Apdative Streaming
    1. Stream JPEG Full Frame
       1. Support 
    2. Implement HLS
       1. https://github.com/oatpp/example-hls-media-stream
       2. 
  - Seperate io_context/thread.
- Generate Track Manager
- Backpressure?
  - Video Stream?
    - What if we can process incoming frames fast encough?
      - Throw out?
    - Can't process Motion Capture fast enough.
  - Motion Updates?
- Animatable Channels?
  - Camera Focal Length
  - Camera Focal Ring
  - Camera Aperture
  - Event Triggers?
- Refactor Logging Header
- Add CMake CI (macOS/Linux)
- mDNS and SD
  - https://github.com/mjansson/mdns
  - https://github.com/gocarlos/mdns_cpp
- SSL Connection
- Server Logging Configuration
  - Logging Levels
  - Logger Names in Output


### Python Bindings Thoughts:
```python
import indiemotion as idm

server = idm.Server()
server.controller = Controller()


```