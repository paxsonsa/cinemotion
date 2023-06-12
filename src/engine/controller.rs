use crate::api;
use crate::Result;

use super::Engine;
use super::ServiceTransport;

pub struct EngineController {
    transport: ServiceTransport,
    shutdown_rx: tokio::sync::mpsc::Receiver<()>,
    tick_control: TickControl,
}

impl EngineController {
    pub fn new(
        transport: ServiceTransport,
        shutdown_rx: tokio::sync::mpsc::Receiver<()>,
        tick_control: TickControl,
    ) -> Self {
        Self {
            transport,
            shutdown_rx,
            tick_control,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("starting engine...");

        let mut command_buffer = Vec::<api::Command>::new();
        let mut engine = Engine::default();
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
                _ = self.tick_control.tick() => {
                    let mut buffer = std::mem::take(&mut command_buffer);
                    for command in buffer.drain(..) {
                        // TODO: Handle errors and send back to client, but continue unless fatal.
                        engine.apply(command).await?;
                    }

                    let state = engine.tick().await?;
                    self.transport.send_state_update(state).await?;
                },
            }
        }

        Ok(())
    }
}

pub enum TickControlBeahvior {
    Interval(tokio::time::Interval),
    #[cfg(test)]
    Channel(tokio::sync::mpsc::Receiver<()>),
}

pub struct TickControl {
    tick_control: TickControlBeahvior,
}

impl TickControl {
    pub fn interval(millis: u64) -> Self {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(millis));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        Self {
            tick_control: TickControlBeahvior::Interval(interval),
        }
    }

    #[cfg(test)]
    pub fn channel() -> (Self, tokio::sync::mpsc::Sender<()>) {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        (
            Self {
                tick_control: TickControlBeahvior::Channel(rx),
            },
            tx,
        )
    }

    async fn tick(&mut self) {
        match &mut self.tick_control {
            TickControlBeahvior::Interval(ref mut interval) => {
                interval.tick().await;
            }
            #[cfg(test)]
            TickControlBeahvior::Channel(ref mut channel) => {
                channel.recv().await;
            }
        }
    }
}
