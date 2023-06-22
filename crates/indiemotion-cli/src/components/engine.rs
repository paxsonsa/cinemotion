use super::Component;
use crate::engine;
use crate::Result;
use async_trait::async_trait;
use std::pin::Pin;

pub struct EngineComponent {
    future: tokio::task::JoinHandle<std::result::Result<(), crate::Error>>,
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl EngineComponent {
    pub fn builder() -> EngineComponentBuilder {
        EngineComponentBuilder::new()
    }
}

#[async_trait]
impl Component for EngineComponent {
    fn name(&self) -> &'static str {
        "engine"
    }

    async fn shutdown(&self) {
        // the error case here is that we are already stopped because
        // the receiver end of the channel has closed, which is ok
        tracing::info!(name = %self.name(), "shutting down");
        let _ = self.shutdown_tx.send(()).await;
    }
}

impl futures::Future for EngineComponent {
    type Output = ();

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        use std::task::Poll::*;

        match Pin::new(&mut self.future).poll(cx) {
            Pending => Pending,
            Ready(Ok(Ok(_))) => {
                tracing::info!(name = %self.name(), "component exited");
                Ready(())
            }
            Ready(Ok(Err(err))) => {
                tracing::info!(%err, name = %self.name(), "component failed");
                Ready(())
            }
            Ready(Err(err)) => {
                tracing::error!(%err, name=%self.name(), "component panic'd");
                Ready(())
            }
        }
    }
}

#[derive(Default)]
pub struct EngineComponentBuilder {
    command_rx: Option<tokio::sync::mpsc::UnboundedReceiver<String>>,
    state_tx: Option<tokio::sync::mpsc::UnboundedSender<String>>,
}

impl EngineComponentBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_command_rx(mut self, tx: tokio::sync::mpsc::UnboundedReceiver<String>) -> Self {
        self.command_rx = Some(tx);
        self
    }

    pub fn with_state_tx(mut self, tx: tokio::sync::mpsc::UnboundedSender<String>) -> Self {
        self.state_tx = Some(tx);
        self
    }

    pub async fn build(self) -> Result<(EngineComponent, engine::Service)> {
        let shutdown_channel = tokio::sync::mpsc::channel(1);
        let tick_control = engine::TickControl::interval(12);

        let (service, transport) = engine::Service::new();
        let mut controller =
            engine::EngineRuntime::new(transport, shutdown_channel.1, tick_control);

        Ok((
            EngineComponent {
                future: tokio::task::spawn(async move {
                    controller.run().await?;
                    Ok(())
                }),
                shutdown_tx: shutdown_channel.0,
            },
            service,
        ))
    }
}
