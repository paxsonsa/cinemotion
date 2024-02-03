use crate::connection::LOCAL_CONN_ID;
use crate::messages::{AddConnection, Message, MessagePipeTx};
use crate::{Error, Result};

use crate::data::WebRTCSessionDescriptor;

use super::WebRTCAgent;

pub struct SignalingRelay {
    sender: MessagePipeTx,
}

impl SignalingRelay {
    pub fn new(sender: MessagePipeTx) -> Self {
        SignalingRelay { sender }
    }

    pub async fn create(
        &self,
        session_desc: WebRTCSessionDescriptor,
    ) -> Result<WebRTCSessionDescriptor> {
        let (ack_pipe, ack_pipe_rx) = tokio::sync::oneshot::channel();

        let (remote_desc, session) = WebRTCAgent::new(session_desc, self.sender.clone()).await?;
        let session = Box::new(session);
        let message = Message::with_command(
            LOCAL_CONN_ID,
            AddConnection {
                agent: session,
                ack_pipe,
            },
        );

        if self.sender.send(message).is_err() {
            return Err(Error::SignalingFailed(
                "lost connection to runtime while attempting to establish session".to_string(),
            ));
        }

        match ack_pipe_rx.await {
            Ok(result) => {
                if let Err(err) = result {
                    tracing::error!("failed to complete signaling message with: {err}");
                    return Err(Error::SignalingFailed(format!(
                        "runtime responded to message with error, {err}"
                    )));
                }
                Ok(remote_desc)
            }
            Err(_) => Err(Error::SignalingFailed(
                "lost connection to runtime while setting up session.".to_string(),
            )),
        }
    }
}
