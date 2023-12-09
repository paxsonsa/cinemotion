use crate::commands::{Command, CreateSession};
use crate::Error;
use crate::{commands::CommandSender, Result};

use crate::data::SessionDescriptor;

#[cfg(test)]
#[path = "./signaling_test.rs"]
mod signaling_test;

pub struct SignalingRelay {
    sender: CommandSender,
}

impl SignalingRelay {
    pub fn new(sender: CommandSender) -> Self {
        SignalingRelay { sender }
    }

    pub async fn create(&self, session_desc: SessionDescriptor) -> Result<SessionDescriptor> {
        let (result_sender, result_receiver) = tokio::sync::oneshot::channel();

        let command: Command = CreateSession {
            session_desc,
            sender: result_sender,
        }
        .into();

        if let Err(err) = self.sender.send(command) {
            return Err(Error::SignalingFailed(
                "lost connection to runtime while attempting to establish session".to_string(),
            ));
        }

        match result_receiver.await {
            Ok(result) => match result {
                Ok(desc) => Ok(desc),
                Err(err) => {
                    tracing::error!("failed to complete signaling request with: {err}");
                    Err(Error::SignalingFailed(format!(
                        "runtime responded to request with error, {err}"
                    )))
                }
            },
            Err(_) => Err(Error::SignalingFailed(
                "lost connection to runtime.".to_string(),
            )),
        }
    }
}
