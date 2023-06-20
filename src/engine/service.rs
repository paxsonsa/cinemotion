use crate::api;
use crate::{Error, Result};

#[derive(Clone)]
pub struct EngineMessage {
    pub client: Option<u32>,
    pub message: api::Message,
}

#[derive(Debug)]
pub struct ClientCommand {
    pub client: u32,
    pub command: api::Command,
}

pub struct Service {
    message_rx: tokio::sync::mpsc::UnboundedReceiver<EngineMessage>,
    command_tx: tokio::sync::mpsc::UnboundedSender<ClientCommand>,
}

impl Service {
    pub fn new() -> (Self, ServiceTransport) {
        let (message_tx, message_rx) = tokio::sync::mpsc::unbounded_channel();
        let (command_tx, command_rx) = tokio::sync::mpsc::unbounded_channel();

        (
            Self {
                message_rx,
                command_tx,
            },
            ServiceTransport {
                message_tx,
                command_rx,
            },
        )
    }

    pub async fn enqueue_command(&mut self, client: u32, command: api::Command) -> Result<()> {
        let command = ClientCommand { client, command };
        self.command_tx
            .send(command)
            .map_err(|_| Error::TransportError("failed to enqueue command"))?;
        Ok(())
    }

    pub async fn recv_message(&mut self) -> Option<EngineMessage> {
        self.message_rx.recv().await
    }
}

pub struct ServiceTransport {
    message_tx: tokio::sync::mpsc::UnboundedSender<EngineMessage>,
    command_rx: tokio::sync::mpsc::UnboundedReceiver<ClientCommand>,
}

impl ServiceTransport {
    pub async fn recv_command(&mut self) -> Option<ClientCommand> {
        self.command_rx.recv().await
    }

    pub async fn send_error(&mut self, client: u32, err: api::Error) -> Result<()> {
        let message = EngineMessage {
            client: Some(client),
            message: api::Message::Error(err),
        };

        self.message_tx
            .send(message)
            .map_err(|_| Error::TransportError("failed to send error"))?;
        Ok(())
    }

    pub async fn send_state_update(&mut self, state: api::GlobalState) -> Result<()> {
        let message = EngineMessage {
            client: None,
            message: api::Message::State(state),
        };

        self.message_tx
            .send(message)
            .map_err(|_| Error::TransportError("failed to send state update"))?;
        Ok(())
    }
}
