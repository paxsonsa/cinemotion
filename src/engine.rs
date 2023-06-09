use crate::{Error, Result};

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
        loop {
            tokio::select! {
                _ = self.shutdown_rx.recv() => {
                    tracing::debug!("engine controller received shutdown, shutting down...");
                    break;
                },
                command = self.transport.recv_command() => {
                    tracing::info!("engine controller received command: {:?}", command);
                }
                _ = interval.tick() => {
                    self.transport.send_state_update("hello".to_string()).await?;
                },
            }
        }

        Ok(())
    }
}

pub struct Service {
    state_rx: tokio::sync::mpsc::UnboundedReceiver<String>,
    command_tx: tokio::sync::mpsc::UnboundedSender<String>,
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

    pub async fn recv_state_update(&mut self) -> Option<String> {
        self.state_rx.recv().await
    }
}

pub struct ServiceTransport {
    state_tx: tokio::sync::mpsc::UnboundedSender<String>,
    command_rx: tokio::sync::mpsc::UnboundedReceiver<String>,
}

impl ServiceTransport {
    pub async fn recv_command(&mut self) -> Option<String> {
        self.command_rx.recv().await
    }

    pub async fn send_state_update(&mut self, state: String) -> Result<()> {
        self.state_tx
            .send(state)
            .map_err(|_| Error::TransportError("failed to send state update"))?;
        Ok(())
    }
}
