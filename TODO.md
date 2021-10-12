- WebSocket Server
    - Listener (accept) [Maybe as Server???]
        - Only Accept Single Connection
        - OnAccept Store Session
    - Connection (read/write)
        When a connection is made thats when the delegates 
        and the SessionManager are created.
        Do we need a queue for the converted messages?

- Responses as Protobuf
- Replace messages with protobufs

- RESET Message
- Add Logging System
- [MAYBE] Use HTTP for all information requests and streamed data via http?
- Generate Track Manager
- Add CMake CI (macOS/Linux)




[Logging Sudo]
``` c++
LOGGER = logging::getLogger("server");
LOGGER.error();

```
