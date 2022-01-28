# IndieMotion Protobuf Message Protocol
The indiemotion message protocol relies on (google's protobuf toolset)[https://github.com/protocolbuffers/protobuf] to make the protocol accessible to multiple contexts and users.
This repository contains the definitions for the standard messages used in indiemotion.

These protobufs define a communication protocol that is used by the two clients to control virtual 
cameras over a network protocols like websockets. An application developer can generate source code 
communicate with a motion server or client application with these.

The protocol itself is designed to allow for near realtime bidirectional communication and streaming of motion data, video
frames, and other metadata between two clients: An input device and some application plugin.

## Getting Started
After clone the repo and selecting a branch/tag. Users can use the protoc commandline tool to build
the source code for their messaging.
```bash
$ cd indiemotion-protobuf
```
### C/C++
The C++ bindings can be generated doing the following.
```
$ protoc -I=. --cpp_out=path/to/dest ./*.proto
```
### Swift
For swift, I have found that making sure the Visibility is `Public` makes working with the output items easier.
Swift is not included by default in the protoc, but [instructions for how to install and use it can be found here.](https://github.com/apple/swift-protobuf)

Compile the swift source code:
```bash
$ protoc -I=. --swift_out=./src/swift/ --swift_opt=Visibility=Public *.proto
```
