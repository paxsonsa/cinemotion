- Translator (message to protobuf)
    - translateMessage
    - translateProtobuf

- WebSocket Server
    - Listener (accept) [Maybe as Server???]
        - Only Accept Single Connection
        - OnAccept Store Session
    - Connection (read/write)
        When a connection is made thats when the delegates 
        and the SessionManager are created.
        Do we need a queue for the converted messages?

- Reset Call
- [MAYBE] Use HTTP for all information requests and streamed data via http?
- Generate Track Manager
- Add CMake CI (macOS/Linux)
- Add Message Queue?
- Messages: add an needs acknowledgement flag so client/server can flag when an ack/error is expected
    - Not sure if its totally nessecary because we can use the ack tracker to emit an error after an ack is not returned

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
