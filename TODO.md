- Create Echo SessionServer Delegate
  - Create SessionServer/SessionConnection
    - MVP: Launch SessionServer and Send Basic Setup with Shutdown. 
    - Need to be able to pass in Delegate Constructor
    - SessionServer takes shared_ptr to delegate.
    - SessionServer Start Up
    - SessionConnection Accepting
    - Process Thread
    
- WebSocket SessionServer
    - SessionConnectionListener (accept) [Maybe as SessionServer???]
        - Only Accept Single SessionConnection
        - OnAccept Store Session
    - SessionConnection (read/write)
        When a connection is made thats when the delegates 
        and the SessionManager are created.
        Do we need a queue for the converted messages?

- SessionProxy
  - Given to SessionControllerDelegate to Control Session and Pass Messages to user.
  - 

- Reset Call
- [MAYBE] Use HTTP for all information requests and streamed data via http?
- Generate Track Manager
- Add CMake CI (macOS/Linux)

[Logging Sudo]
``` c++
LOGGER = logging::getLogger("server");
LOGGER.error();

```
[Protobuf Session Handler]
```c++

class SessionConnection {
...
// SessionConnection will readin protobuf message and wrapp it inside of a message wrapper
// The messageWrapper can be used to determine the handler to use for the message.
// Essentially, nothing changes for the message handlers except the messages become message wrappers
protobuf::messages::ClientMessage message;
message.ParseFromString(buffer.data());
auto wrapperPtr = messageWrapperFactory.make(message);
auto responseWrapperPtr = manager.processMessage(std::move(wrapperPtr));
auto response = responseProtobufFactory.from(std::move(responseWrapperPtr));
sendResponse(response);
...
}
// Responses are a bit more interesting 

```
