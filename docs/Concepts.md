# Core Concepts
IndieMotion breaks camera motion capture into a few components to make recording camera
motion agnostic between DCCs.

## Sessions
Each connection instance is referred as a 'session'. You can think of
these as individual recording sessions/shoots. On the device the session
is used to group motion tracks and data. During a session, a camera will be selected and live
motion data will be transmitted to the server.

## Cameras
Obviously, the framework has the concept of a camera. A session records motion to a selected camera.
A camera is a simple object in this framework, just consisting of an identifier that is used
for selecting and updating.

## Tracks
A track is a set of samples for individual channels for a given session and camera. The tracks are
saved on the external device and when a recording is completed transmitted to the server
to be applied. 

# 