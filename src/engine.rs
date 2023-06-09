use crate::Result;

pub struct EngineController {
    state_tx: tokio::sync::mpsc::UnboundedSender<String>,
    command_rx: tokio::sync::mpsc::UnboundedReceiver<String>,
    shutdown_rx: tokio::sync::mpsc::Receiver<()>,
}

impl EngineController {
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

pub struct EngineProxy {
    command_tx: tokio::sync::mpsc::UnboundedSender<String>,
    state_rx: tokio::sync::mpsc::UnboundedReceiver<String>,
}
