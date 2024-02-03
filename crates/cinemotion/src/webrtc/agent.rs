use crate::{
    connection::SendHandlerFn,
    data::WebRTCSessionDescriptor,
    messages::{self, MessagePipeTx, Payload},
    Error, Event, Result,
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

use crate::connection::ConnectionAgent;

pub struct WebRTCAgent {
    peer_connection: Arc<RTCPeerConnection>,
    send_handler: Arc<ArcSwapOption<Mutex<SendHandlerFn>>>,
    main_channel: Option<Arc<RTCDataChannel>>,
}

impl WebRTCAgent {
    /// Establish a new WebRTC based session
    ///
    /// Returns the session descriptor to send back to client and an active session.
    pub async fn new(
        desc: WebRTCSessionDescriptor,
        message_pipe: MessagePipeTx,
    ) -> Result<(WebRTCSessionDescriptor, Self)> {
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
            Some(desc) => WebRTCSessionDescriptor::new(&desc.sdp),
            None => {
                return Err(Error::ConnectionFailed(
                    "failed to establish webrtc peer connection".to_string(),
                ))
            }
        };

        Ok((
            local_desc,
            Self {
                peer_connection,
                send_handler: Default::default(),
                main_channel: None,
            },
        ))
    }
}

#[async_trait]
impl ConnectionAgent for WebRTCAgent {
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
                    let init = messages::OpenConnection {};
                    let _ = f(init.into());
                }
            })
        }));

        // Establish the session close sequence once the data channel is closed
        let shared_send_fn = Arc::clone(&self.send_handler);
        main_channel.on_close(Box::new(move || {
            let shared_send_fn = Arc::clone(&shared_send_fn);

            Box::pin(async move {
                let command = messages::CloseConnection {};
                tracing::debug!("agent data channel closed");
                if let Some(handler) = &*shared_send_fn.load() {
                    let mut f = handler.lock().await;
                    let _ = f(command.into());
                }
            })
        }));

        // Establish the message handling when the data channel receives a message
        let shared_send_fn = Arc::clone(&self.send_handler);
        let shared_channel = Arc::clone(&main_channel);
        main_channel.on_message(Box::new(move |msg| {
            // Decode the incoming message as a command and send it through the handler.
            let shared_send_fn = Arc::clone(&shared_send_fn);
            let shared_channel = Arc::clone(&shared_channel);
            Box::pin(async move {
                let command = match Payload::from_protobuf_bytes(msg.data) {
                    Ok(command) => command,
                    Err(err) => {
                        tracing::error!("failed to decode command. err={err}");
                        let error = make_error_event(err);
                        convert_and_send(&shared_channel, error).await;
                        return;
                    }
                };

                if let Some(handler) = &*shared_send_fn.load() {
                    let mut f = handler.lock().await;
                    let _ = f(command);
                }
            })
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

        // Save the main channel
        self.main_channel = Some(main_channel);
    }
    async fn receive(&mut self, event: Event) {
        let Some(channel) = &self.main_channel else {
            return;
        };
        convert_and_send(channel, event).await;
    }
    async fn close(&mut self) {
        let _ = self.peer_connection.close().await;
    }
}

fn make_error_event(err: Error) -> crate::Event {
    crate::Event {
        target: Some(0),
        body: crate::events::ErrorEvent(err).into(),
    }
}

async fn convert_and_send(channel: &Arc<RTCDataChannel>, event: crate::Event) {
    let proto: cinemotion_proto::Event = event.into();
    let data = match proto.try_into() {
        Ok(data) => data,
        Err(err) => {
            tracing::error!("failed to encode event. err={err}");
            return;
        }
    };
    if let Err(err) = channel.send(&data).await {
        tracing::error!("failed to send event. err={err}");
    }
}
