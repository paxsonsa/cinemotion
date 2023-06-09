use std::collections::HashMap;

use crate::api;
use crate::{engine, sync};
use crate::{Error, Result};

pub type ProxyCommandsTx = tokio::sync::mpsc::UnboundedSender<Command>;
pub type ProxyCommandsRx = tokio::sync::mpsc::UnboundedReceiver<Command>;
pub fn proxy_channel() -> (ProxyCommandsTx, ProxyCommandsRx) {
    tokio::sync::mpsc::unbounded_channel()
}

pub struct ClientManager {
    engine: engine::Service,
    command_rx: crate::clients::ProxyCommandsRx,
    shutdown_rx: tokio::sync::mpsc::Receiver<()>,
    clients: HashMap<u32, Client>,
    next_handle: u32,
}

impl ClientManager {
    pub fn new(
        engine: engine::Service,
        command_rx: crate::clients::ProxyCommandsRx,
        shutdown_rx: tokio::sync::mpsc::Receiver<()>,
    ) -> Self {
        Self {
            engine,
            command_rx,
            shutdown_rx,
            clients: HashMap::new(),
            next_handle: 0,
        }
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
                            tracing::error!("client manager error'd will processing command: {}", e);
                        }
                    }
                },
                state = self.engine.recv_state_update() => {
                    // TODO Broadcast to clients.
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
            Command::Connect(client, result_tx) => {
                let _ = result_tx.send(self.add_client(client).await);
            }
            Command::Disconnect(handle, result_tx) => {
                let _ = result_tx.send(self.remove_client(handle).await);
            }
            Command::ReceiveFrom(handle, command, result_tx) => {
                tracing::info!("client relay controller received receive from command");
                let _ = result_tx.send(self.receive_from(handle, command).await);
            }
        }

        Ok(())
    }

    async fn add_client(&mut self, client: Client) -> Result<u32> {
        let handle = self.next_handle();
        self.clients.insert(handle, client);
        Ok(handle)
    }

    async fn remove_client(&mut self, handle: u32) -> Result<()> {
        self.clients.remove(&handle);
        Ok(())
    }

    async fn receive_from(&mut self, handle: u32, command: api::Command) -> Result<()> {
        tracing::info!("receive_from({}, {:?}", handle, command);
        let _ = match self.clients.get_mut(&handle) {
            Some(client) => client,
            None => {
                tracing::error!("client not found for handle: {}", handle);
                return Err(Error::ClientNotFound(handle));
            }
        };

        self.engine.enqueue_command(command).await?;

        // client.send(api::State {}).await?;
        Ok(())
    }

    fn next_handle(&mut self) -> u32 {
        let handle = self.next_handle;
        self.next_handle += 1;
        handle
    }
}

#[derive(Debug)]
pub enum Command {
    Connect(Client, sync::ResultTx<u32>),
    Disconnect(u32, sync::ResultTx<()>),
    ReceiveFrom(u32, api::Command, sync::ResultTx<()>),
}

#[derive(Debug, Clone)]
pub struct ClientService {
    command_tx: tokio::sync::mpsc::UnboundedSender<Command>,
}

impl ClientService {
    pub fn new(command_tx: tokio::sync::mpsc::UnboundedSender<Command>) -> Self {
        Self { command_tx }
    }

    pub async fn connect(&self, client: Client) -> Result<u32> {
        let (tx, result) = sync::result::<u32>();
        self.command_tx
            .send(Command::Connect(client, tx))
            .map_err(|_| Error::ClientError("failed to register client".to_string()))?;

        result
            .await
            .map_err(|_| Error::ClientError("failed to register client".to_string()))?
    }

    pub async fn disconnect(&self, client_handle: u32) -> Result<()> {
        let (tx, result) = sync::result::<()>();
        self.command_tx
            .send(Command::Disconnect(client_handle, tx))
            .map_err(|_| Error::ClientError("failed to disconnect client".to_string()))?;

        result
            .await
            .map_err(|_| Error::ClientError("failed to disconnect client".to_string()))?
    }

    pub async fn receive_from(&self, client_handle: u32, command: api::Command) -> Result<()> {
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
    state_channel: tokio::sync::mpsc::UnboundedSender<api::State>,
}

impl Client {
    pub fn new(state_channel: tokio::sync::mpsc::UnboundedSender<api::State>) -> Self {
        Self { state_channel }
    }

    pub async fn send(&mut self, state: api::State) -> Result<()> {
        self.state_channel
            .send(state)
            .map_err(|_| Error::ClientError("failed to send message".to_string()))
    }
}
