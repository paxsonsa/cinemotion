use crate::{commands::RequestPipeTx, data::SessionDescriptor, Error, Result};
use std::sync::Arc;
use webrtc::{
    api::{media_engine::MediaEngine, APIBuilder},
    data_channel::{data_channel_init::RTCDataChannelInit, RTCDataChannel},
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription, RTCPeerConnection,
    },
};

use crate::session::Session;

pub struct WebRTCSession {
    peer_connection: Arc<RTCPeerConnection>,
}

impl WebRTCSession {
    /// Establish a new WebRTC based session
    ///
    /// Returns the session descriptor to send back to client and an active session.
    pub async fn new(desc: SessionDescriptor) -> Result<(SessionDescriptor, Self)> {
        let m = MediaEngine::default();
        let api = APIBuilder::new().with_media_engine(m).build();

        let config = RTCConfiguration {
            ..Default::default()
        };

        let peer_connection = Arc::new(api.new_peer_connection(config).await?);

        let offer = RTCSessionDescription::offer(desc.decode()?)?;

        // Set the remote SessionDescription
        peer_connection.set_remote_description(offer).await?;

        // Create an answer
        let answer = peer_connection.create_answer(None).await?;

        // Create channel that is blocked until ICE Gathering is complete
        let mut gather_complete = peer_connection.gathering_complete_promise().await;

        // Sets the LocalDescription, and starts our UDP listeners
        peer_connection.set_local_description(answer).await?;

        // Block until ICE Gathering is complete, disabling trickle ICE
        // we do this because we only can exchange one signaling message
        let _ = gather_complete.recv().await;

        let local_desc = match peer_connection.local_description().await {
            Some(desc) => SessionDescriptor::new(&desc.sdp),
            None => {
                return Err(Error::SessionFailed(
                    "failed to establish webrtc peer connection",
                ))
            }
        };

        Ok((local_desc, Self { peer_connection }))
    }
}

impl Session for WebRTCSession {}
