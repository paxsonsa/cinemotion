# indiemotionpb - IndieMotion Protobuf C++ Source
This subproject is automatically generated using the [IndieMotion Protobuf Repository Tag v1.0.1](https://github.com/paxsonsa/indiemotion-protobuf/tree/v1.0.1)

Developers should avoid touching the source code manually and instead following the instructions for
the protobuf generator.

### Generating the source code
```bash
$ cd source/indiemotionpb
$ protoc -I=/path/to/indiemotion-protobufs/ --cpp_out=./include/indiemotionpb /path/to/indiemotion-protobufs/*.proto
```