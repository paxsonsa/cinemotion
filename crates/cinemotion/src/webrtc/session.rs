use crate::{
    commands::{self, Command, Event, EventPipeRx, RequestPipeTx},
    data::SessionDescriptor,
    session::SendHandlerFn,
    Error, Result,
};

use arc_swap::ArcSwapOption;
use async_trait::async_trait;
use futures::lock::Mutex;
use std::sync::Arc;
use webrtc::{
    api::{media_engine::MediaEngine, APIBuilder},
    data_channel::{data_channel_init::RTCDataChannelInit, RTCDataChannel},
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription, RTCPeerConnection,
    },
};

use crate::session::SessionAgent;

pub struct WebRTCAgent {
    peer_connection: Arc<RTCPeerConnection>,
    send_handler: Arc<ArcSwapOption<Mutex<SendHandlerFn>>>,
}

impl WebRTCAgent {
    /// Establish a new WebRTC based session
    ///
    /// Returns the session descriptor to send back to client and an active session.
    pub async fn new(
        desc: SessionDescriptor,
        request_pipe: RequestPipeTx,
    ) -> Result<(SessionDescriptor, Self)> {
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
                    "failed to establish webrtc peer connection".to_string(),
                ))
            }
        };

        Ok((
            local_desc,
            Self {
                peer_connection,
                send_handler: Default::default(),
            },
        ))
    }
}

#[async_trait]
impl SessionAgent for WebRTCAgent {
    async fn initialize(&mut self, send_fn: SendHandlerFn) {
        // Bind the given send handler for use in the session
        self.send_handler.store(Some(Arc::new(Mutex::new(send_fn))));
        let options = RTCDataChannelInit {
            ordered: Some(true),
            ..Default::default()
        };

        // Create the main channel for sending and receiving data
        let main_channel = match self
            .peer_connection
            .create_data_channel("main", Some(options))
            .await
        {
            Ok(c) => c,
            Err(err) => {
                tracing::error!("failed to create agent data channel. err={err}");
                return;
            }
        };

        // Establish the session open sequence once the data channel is opened
        let shared_send_fn = Arc::clone(&self.send_handler);
        main_channel.on_open(Box::new(move || {
            Box::pin(async move {
                if let Some(handler) = &*shared_send_fn.load() {
                    let mut f = handler.lock().await;
                    let init = commands::OpenSession {};
                    let _ = f(init.into());
                }
            })
        }));

        // Establish the message handling when the data channel receives a message
        main_channel.on_message(Box::new(move |msg| {
            // TODO: use send handler to receive bytes and convert from protobuf
            // TODO:: Protobuf Echo

            Box::pin(async {})
        }));

        // Listen for peer connection state changes
        self.peer_connection
            .on_peer_connection_state_change(Box::new(move |state: RTCPeerConnectionState| {
                match state {
                    RTCPeerConnectionState::Unspecified => {}
                    RTCPeerConnectionState::New => {
                        tracing::debug!("agent connection in new state");
                    }
                    RTCPeerConnectionState::Connecting => {
                        tracing::debug!("agent is connecting");
                    }
                    RTCPeerConnectionState::Connected => {
                        tracing::debug!("agent is connected");
                    }
                    RTCPeerConnectionState::Disconnected => {
                        tracing::warn!("agent is disconnected, may come back");
                    }
                    RTCPeerConnectionState::Failed => {
                        tracing::error!("agent connection has failed.");
                    }
                    RTCPeerConnectionState::Closed => {
                        tracing::error!("agent connection was closed.");
                    }
                };

                Box::pin(async {})
            }));
    }
    async fn receive(&mut self, event: Event) {}
    fn close(&mut self) {}
}
