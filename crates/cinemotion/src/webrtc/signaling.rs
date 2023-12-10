use crate::commands::{CreateSession, Request, RequestPipeTx};
use crate::{Error, Result};

use crate::data::SessionDescriptor;

use super::WebRTCSession;

pub struct SignalingRelay {
    sender: RequestPipeTx,
}

impl SignalingRelay {
    pub fn new(sender: RequestPipeTx) -> Self {
        SignalingRelay { sender }
    }

    pub async fn create(&self, session_desc: SessionDescriptor) -> Result<SessionDescriptor> {
        let (ack_pipe, ack_pipe_rx) = tokio::sync::oneshot::channel();

        let (remote_desc, session) = WebRTCSession::new(session_desc).await?;
        let session = Box::new(session);
        let request = Request::with_command(CreateSession { session, ack_pipe });

        if self.sender.send(request).is_err() {
            return Err(Error::SignalingFailed(
                "lost connection to runtime while attempting to establish session".to_string(),
            ));
        }

        match ack_pipe_rx.await {
            Ok(result) => {
                if let Err(err) = result {
                    tracing::error!("failed to complete signaling request with: {err}");
                    return Err(Error::SignalingFailed(format!(
                        "runtime responded to request with error, {err}"
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
