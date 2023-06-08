use std::collections::HashMap;

use crate::sync;
use crate::{Error, Result};

pub type ProxyCommandsTx = tokio::sync::mpsc::UnboundedSender<Command>;
pub type ProxyCommandsRx = tokio::sync::mpsc::UnboundedReceiver<Command>;
pub fn proxy_channel() -> (ProxyCommandsTx, ProxyCommandsRx) {
    tokio::sync::mpsc::unbounded_channel()
}

pub struct ClientRelayController {
    // TODO - Turn Command TX in EngineProxy
    command_tx: tokio::sync::mpsc::UnboundedSender<String>,
    command_rx: crate::clients::ProxyCommandsRx,
    shutdown_rx: tokio::sync::mpsc::Receiver<()>,
    state_rx: tokio::sync::mpsc::UnboundedReceiver<String>,
    clients: HashMap<u32, Client>,
    next_handle: u32,
}

impl ClientRelayController {
    pub fn builder() -> ClientRelayBuilder {
        ClientRelayBuilder::new()
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("starting client relay...");
        loop {
            tokio::select! {
                command = self.command_rx.recv() => {

                    let command = match command {
                        Some(command) => command,
                        None => {
                            tracing::debug!("client relay controller command channel closed unexpected, shutting component down...");
                            break;
                        }
                    };
                    match self.process_command(command).await {
                        Ok(_) => (),
                        Err(e) => {
                            tracing::error!("client relay controller command error: {}", e);
                            // TODO Send Error to client.
                            break;
                        }
                    }
                },
                state = self.state_rx.recv() => {
                    tracing::info!("client relay controller received state update: {:?}", state);
                },
                _ = self.shutdown_rx.recv() => {
                    tracing::debug!("client relay controller received shutdown, shutting down...");
                    break;
                },
            }
        }
        Ok(())
    }

    async fn process_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::ConnectClient(client, result_tx) => {
                // TODO Handle Send Errors back to client pop from clients
                let _ = result_tx.send(self.connect_client(client).await);
            }
            Command::ReceiveFrom(handle, message, result_tx) => {
                tracing::info!("client relay controller received receive from command");
                let _ = result_tx.send(self.receive_from(handle, message).await);
            }
        }

        Ok(())
    }

    async fn connect_client(&mut self, client: Client) -> Result<u32> {
        tracing::info!("connect_client()");
        let handle = self.next_handle();
        self.clients.insert(handle, client);
        Ok(handle)
    }

    async fn receive_from(&mut self, handle: u32, message: String) -> Result<()> {
        tracing::info!("receive_from()");
        let client = match self.clients.get_mut(&handle) {
            Some(client) => client,
            None => {
                tracing::error!("client not found for handle: {}", handle);
                return Err(Error::ClientNotFound(handle));
            }
        };
        client.send(message).await?;
        Ok(())
    }

    fn next_handle(&mut self) -> u32 {
        let handle = self.next_handle;
        self.next_handle += 1;
        handle
    }
}

#[derive(Default)]
pub struct ClientRelayBuilder {
    shutdown_tx: Option<tokio::sync::mpsc::Receiver<()>>,
    command_rx: Option<crate::clients::ProxyCommandsRx>,
    command_tx: Option<tokio::sync::mpsc::UnboundedSender<String>>,
    state_rx: Option<tokio::sync::mpsc::UnboundedReceiver<String>>,
}

impl ClientRelayBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> ClientRelayController {
        ClientRelayController {
            command_rx: self.command_rx.unwrap(),
            command_tx: self.command_tx.unwrap(),
            shutdown_rx: self.shutdown_tx.unwrap(),
            state_rx: self.state_rx.unwrap(),
            clients: Default::default(),
            next_handle: 0,
        }
    }

    pub fn with_command_rx(mut self, rx: crate::clients::ProxyCommandsRx) -> Self {
        self.command_rx = Some(rx);
        self
    }

    pub fn with_command_tx(mut self, tx: tokio::sync::mpsc::UnboundedSender<String>) -> Self {
        self.command_tx = Some(tx);
        self
    }

    pub fn with_state_rx(mut self, rx: tokio::sync::mpsc::UnboundedReceiver<String>) -> Self {
        self.state_rx = Some(rx);
        self
    }

    pub fn with_shutdown_rx(mut self, tx: tokio::sync::mpsc::Receiver<()>) -> Self {
        self.shutdown_tx = Some(tx);
        self
    }
}

#[derive(Debug)]
pub enum Command {
    ConnectClient(Client, sync::ResultTx<u32>),
    ReceiveFrom(u32, String, sync::ResultTx<()>),
}

#[derive(Debug, Clone)]
pub struct ClientRelayProxy {
    command_tx: tokio::sync::mpsc::UnboundedSender<Command>,
}

impl ClientRelayProxy {
    pub fn new(command_tx: tokio::sync::mpsc::UnboundedSender<Command>) -> Self {
        Self { command_tx }
    }

    pub async fn register_client(&self, client: Client) -> Result<u32> {
        let (tx, result) = sync::result::<u32>();
        self.command_tx
            .send(Command::ConnectClient(client, tx))
            .map_err(|_| Error::ClientError("failed to register client".to_string()))?;

        result
            .await
            .map_err(|_| Error::ClientError("failed to register client".to_string()))?
    }

    pub async fn receive_from(&self, client_handle: u32, command: String) -> Result<()> {
        let (tx, result) = sync::result::<()>();
        self.command_tx
            .send(Command::ReceiveFrom(client_handle, command, tx))
            .map_err(|_| Error::ClientError("failed to receive command".to_string()))?;

        result
            .await
            .map_err(|_| Error::ClientError("failed to receive command".to_string()))?
    }
}

#[derive(Debug)]
pub struct Client {
    message_tx: tokio::sync::mpsc::UnboundedSender<String>,
}

impl Client {
    pub fn new(message_tx: tokio::sync::mpsc::UnboundedSender<String>) -> Self {
        Self { message_tx }
    }

    pub async fn send(&mut self, message: String) -> Result<()> {
        self.message_tx
            .send(message)
            .map_err(|_| Error::ClientError("failed to send message".to_string()))
    }
}
