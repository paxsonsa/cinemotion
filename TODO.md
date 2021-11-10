- Server Start Up
- Connection Accepting
- Process Thread
- Connection Loop 
  - Read
  - Write NetMessage from Send Loop

- WebSocket Server
    - Listener (accept) [Maybe as Server???]
        - Only Accept Single Connection
        - OnAccept Store Session
    - Connection (read/write)
        When a connection is made thats when the delegates 
        and the SessionManager are created.
        Do we need a queue for the converted messages?

- SessionProxy
  - Given to SessionControllerDelegate to Control Session and Pass Messages to user.

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

class Connection {
...
// Connection will readin protobuf message and wrapp it inside of a message wrapper
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
