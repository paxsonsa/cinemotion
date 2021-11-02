# IndieMotion Server Engine 
A framework for recording virtual camera motion into a DCC.


### Building Protobuf Source
``` bash
protoc -I=./protobufs --cpp_out=source/indiemotion-protobufs/include/indiemotion-protobufs protobufs/*.proto
```