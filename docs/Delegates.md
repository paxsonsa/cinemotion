# SessionCon Controller Delegates

### NOTE: OUT OF DATE


The `indiemotion::Application` is the core interface for hooking into your DCC. 
The header file `indiemotion/session/delegate.hpp` contains the most detailed and up-to-date documentation
about the different members to the class.

This document to describs the intended lifecycle for a session and what you can expect.

## SessionCon Start Up
Once a connection is established the session will begin to start up.
At that moment the delegates `will_start_session()` member will be invoked and it should be used
to set up the DCC to begin accepting the sessions input.

Once the session is configured and initialized the delegates `did_start_session()` will be invoked and it can
be used to finalize your apps setup.

At this point the session is considered active.

## Camera Setup
A session cannot begin recording until it has set an active camera. You should expect the following members 
to be invoked immediately following the startup process:

`get_available_cameras` will be called almost immediately, the user cannot select a camera until it knows
what is available. The `Camera` structure itself is very simple for right now, only consisting of an name/id.
The vector of cameras returned will be passed to the client.

`get_camera_by_name` will be the next member invoked. This called is invoked when the client selects a camera
to verify the camera still exists. If it does not, just return an empty optional `{}` and the client
receive a client not found error.

`did_set_active_camera` if the last call and it receives the camera that is being used as the active camera. 
This should be the camera that responds to motion updates and view capture.

## Motion Setup
The core purpose of this framework is to allow for motion to be recorded and transmitted to the session.
The session has a set of motion modes:
- `indiemotion::MotionCaptureMode::Idle` - No motion is being transmitted.
- `indiemotion::MotionCaptureMode::Live` - Motion is being transmitted BUT this information is ephemeral and is not recorded.
- `indiemotion::MotionCaptureMode::Recording` - Motion is being transmitted and recording

The delegate receives changes in these modes via the `did_set_motion_mode` member which will 
provide the new motion mode being set.

## Receiving Motion Updates
If the session's motion mode is set to `MotionCaptureMode::Live` or `MotionCaptureMode::Recording` then your delegate will 
receive motion transform updates through the `did_receive_motion_update` member.
Depending on the session properties, you will receive motion updates very rapidly which are expected to be applied to
the active camera.
 

The `indiemotion::MotionXForm` instance is a very simple object at this time. The translation and rotation are represented
in the `translation` and `orientation` members respectively.

- `translation` member is a left-handed XYZ sample of how far from the initial origin the camera/object is.
- `orientation` member is a left-handed XYZ unit vector of how far from the initial orientation the camera/object is.

### Initial Origin
The initial origin from which motion is measure simply the position the camera is at whenever session transition
from `MotionCaptureMode::Idle` to live or recording. At the exact moment motion is enabled, that position become the initial origin.

Because of the nature of some devices, using an absolute position in the real world is not a viable solution 
(accelerometer based devices). It is on the delegate and DCC to record the initial origin position and 
update the transformation of the camera relative to that frame of reference.

When the motion is disabled the current translation is lost thrown out.

#### Expected User Experience
When motion is enabled the camera should move (duh) but when motion is disabled the camera position SHOULD NOT RESET!
The camera position should only be reset once the delegates `did_reset_camera` is invoked.

The purpose of this framework (and accompanying app) is to allow indie artists to use tools for virtual production so
it is expected they may not have a large room to move around in. The user may start the session sitting down but 
then move to another area to begin recording. The user may want to position the camera to their desire position before
recording.

# Errors and Exceptions
Errors happen. From your delegate we have provided a helper exception to convey to the client user 
problems on the server. 

The `indiemotion::SessionException` exception can be used to relay error description to the user. By specifying `is_fatal` 
to the exception, the session will begin its shutdown sequence. 

# SessionCon Shutdown
Nothing lasts forever. When the session begins to shutdown, the `will_shutdown_session` member will be invoked.
At this stage the session is closing (and the connection could terminate). Your delegate should ensure a clean up of
the application.
