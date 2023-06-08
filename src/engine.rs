use crate::{api, Error, Result};

pub struct EngineController {
    state_tx: tokio::sync::mpsc::UnboundedSender<String>,
    command_rx: tokio::sync::mpsc::UnboundedReceiver<String>,
    shutdown_rx: tokio::sync::mpsc::Receiver<()>,
}

impl EngineController {
    pub fn builder() -> EngineControllerBuilder {
        EngineControllerBuilder::new()
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
                command = self.command_rx.recv() => {
                    tracing::info!("engine controller received command: {:?}", command);
                }
                _ = interval.tick() => {},
            }
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct EngineControllerBuilder {
    command_rx: Option<tokio::sync::mpsc::UnboundedReceiver<String>>,
    state_tx: Option<tokio::sync::mpsc::UnboundedSender<String>>,
    shutdown_rx: Option<tokio::sync::mpsc::Receiver<()>>,
}

impl EngineControllerBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> EngineController {
        EngineController {
            state_tx: self.state_tx.unwrap(),
            command_rx: self.command_rx.unwrap(),
            shutdown_rx: self.shutdown_rx.unwrap(),
        }
    }

    pub fn with_command_rx(mut self, rx: tokio::sync::mpsc::UnboundedReceiver<String>) -> Self {
        self.command_rx = Some(rx);
        self
    }

    pub fn with_state_tx(mut self, tx: tokio::sync::mpsc::UnboundedSender<String>) -> Self {
        self.state_tx = Some(tx);
        self
    }

    pub fn with_shutdown_rx(mut self, tx: tokio::sync::mpsc::Receiver<()>) -> Self {
        self.shutdown_rx = Some(tx);
        self
    }
}
