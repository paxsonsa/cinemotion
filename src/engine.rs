use crate::{Error, Result};

use crate::api;

pub struct EngineController {
    transport: ServiceTransport,
    shutdown_rx: tokio::sync::mpsc::Receiver<()>,
}

impl EngineController {
    pub fn new(transport: ServiceTransport, shutdown_rx: tokio::sync::mpsc::Receiver<()>) -> Self {
        Self {
            transport,
            shutdown_rx,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("starting engine...");
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(12));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut command_buffer = Vec::<api::Command>::new();
        // TODO: Loop Analyics.
        loop {
            tokio::select! {
                _ = self.shutdown_rx.recv() => {
                    tracing::debug!("engine controller received shutdown, shutting down...");
                    break;
                },
                command = self.transport.recv_command() => {
                    let Some(command) = command else {
                        tracing::error!("engine controller service transport closed, shutting down...");
                        break;
                    };

                    tracing::info!("engine controller received command: {:?}", command);
                    command_buffer.push(command);
                }
                _ = interval.tick() => {
                    let buffer = std::mem::take(&mut command_buffer);
                    for command in buffer.iter() {
                        tracing::info!("processing command: {:?}", command);
                    }
                    self.transport.send_state_update(api::State {}).await?;
                },
            }
        }

        Ok(())
    }
}

pub struct Service {
    state_rx: tokio::sync::mpsc::UnboundedReceiver<api::State>,
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

    pub async fn recv_state_update(&mut self) -> Option<api::State> {
        self.state_rx.recv().await
    }
}

pub struct ServiceTransport {
    state_tx: tokio::sync::mpsc::UnboundedSender<api::State>,
    command_rx: tokio::sync::mpsc::UnboundedReceiver<api::Command>,
}

impl ServiceTransport {
    pub async fn recv_command(&mut self) -> Option<api::Command> {
        self.command_rx.recv().await
    }

    pub async fn send_state_update(&mut self, state: api::State) -> Result<()> {
        self.state_tx
            .send(state)
            .map_err(|_| Error::TransportError("failed to send state update"))?;
        Ok(())
    }
}
