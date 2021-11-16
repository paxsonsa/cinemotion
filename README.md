# IndieMotion SessionServer Engine 
A framework for recording virtual camera motion into a DCC.


### Building Protobuf Source
``` bash
protoc -I=./protobufs --cpp_out=source/indiemotion-protobufs/include/indiemotion-protobufs protobufs/*.proto
```

## Build
### External Dependencies
Indiemotion is conformant with the [VFX 2022 Reference](https://vfxplatform.com) Platform. 

| Library | Version | Included? |
| ------------ | ------------- | ----------|
boost | 1.76.0 | No
protobuf3-cpp | 3.17.3 | No
spdlog | 1.9.2 | yes |
fmt | 8.0.1| yes |
replx | 0.0.4 | yes |
doctest | 2.4.6 | yes|
