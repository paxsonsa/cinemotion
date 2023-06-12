use crate::api;
use crate::{Error, Result};

pub struct Service {
    state_rx: tokio::sync::mpsc::UnboundedReceiver<api::GlobalState>,
    command_tx: tokio::sync::mpsc::UnboundedSender<api::Command>,
}

impl Service {
    pub fn new() -> (Self, ServiceTransport) {
        let (state_tx, state_rx) = tokio::sync::mpsc::unbounded_channel();
        let (command_tx, command_rx) = tokio::sync::mpsc::unbounded_channel();

        (
            Self {
                state_rx,
                command_tx,
            },
            ServiceTransport {
                state_tx,
                command_rx,
            },
        )
    }

    pub async fn enqueue_command(&mut self, command: api::Command) -> Result<()> {
        self.command_tx
            .send(command)
            .map_err(|_| Error::TransportError("failed to enqueue command"))?;
        Ok(())
    }

    pub async fn recv_state_update(&mut self) -> Option<api::GlobalState> {
        self.state_rx.recv().await
    }
}

pub struct ServiceTransport {
    state_tx: tokio::sync::mpsc::UnboundedSender<api::GlobalState>,
    command_rx: tokio::sync::mpsc::UnboundedReceiver<api::Command>,
}

impl ServiceTransport {
    pub async fn recv_command(&mut self) -> Option<api::Command> {
        self.command_rx.recv().await
    }

    pub async fn send_state_update(&mut self, state: api::GlobalState) -> Result<()> {
        self.state_tx
            .send(state)
            .map_err(|_| Error::TransportError("failed to send state update"))?;
        Ok(())
    }
}
