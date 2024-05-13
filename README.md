# CineMotion: Device Motion Streaming Framework
In the age of AR/VR, Virtual Production, and Computer Graphics, translating motion from the real world into the computer world is becoming a great tool for creative workflows. 
Unfortunately, existing motion capture systems are pricey, integrations to existing software you use may no exist and you may want to transmit motion from multiple devices and systems.

CineMotion is a framework/protocol that:
1) Helps embed motion streaming from devices in DCC application more easily.
2) A protocol allowing for multiple devices to stream motion data together.
3) Help integrators by avoiding worrying about the networking connectivity integration.
4) Defines a network transport so new devices can easily integrate.

# Benefits
- Most 'motion streaming' integrations are application specific (e.g. just for Unreal, Maya etc). CineMotion is DCC agnostic.
- No device/Application lock in: Many existing motion streaming tools require you to pay for an app or some hardware. CineMotion does not, it allows you to integrate your own device/software to stream data.
- Simple API for implementing python integrations (the norm in VFX/Games tools) or use the rust, c++, or swift bindings.

