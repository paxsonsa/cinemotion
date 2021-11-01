- Error TODOs
- Add Logging System
- Initialization
    - Delegate
        - Session Initialization
        - Session Name
    - API Version from Lib
- Converters (message to protobuf)
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

[Logging Sudo]
``` c++
LOGGER = logging::getLogger("server");
LOGGER.error();

```
[Protobuf Session Handler]
```c++

class MessageWrapper
{
    MessageKind kind;
    class Header
    {
        Timestamp timestamp;
        string id;
        std::optional<string> inResponseToId;
    }

    class Contents
    {

    }

    std::unique_ptr<Header> header
    std::unique_ptr<Contents> payload;
};


class ResponseWrapper
{
    Response kind;
    class Header
    {
        Timestamp
    }
}

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
