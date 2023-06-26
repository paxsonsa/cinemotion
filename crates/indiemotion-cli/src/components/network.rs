use super::Component;
use crate::Result;
use async_trait::async_trait;
use std::pin::Pin;

pub struct NetworkComponent {
    future: tokio::task::JoinHandle<std::result::Result<(), crate::Error>>,
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl NetworkComponent {
    pub fn build(name: String, port: u16) -> NetworkComponentBuilder {
        NetworkComponentBuilder { name, port }
    }
}

#[async_trait]
impl Component for NetworkComponent {
    fn name(&self) -> &'static str {
        "network"
    }

    async fn shutdown(&self) {
        // the error case here is that we are already stopped because
        // the receiver end of the channel has closed, which is ok
        tracing::info!(name = %self.name(), "shutting down");
        let _ = self.shutdown_tx.send(()).await;
    }
}

impl futures::Future for NetworkComponent {
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

pub struct NetworkComponentBuilder {
    name: String,
    port: u16,
}

impl NetworkComponentBuilder {
    pub async fn build(self) -> Result<NetworkComponent> {
        let mut shutdown_channel = tokio::sync::mpsc::channel(1);

        Ok(NetworkComponent {
            future: tokio::task::spawn(async move {
                let responder = libmdns::Responder::new().unwrap();
                let _svc = responder.register(
                    "_http._tcp".to_owned(),
                    self.name,
                    self.port,
                    &["websocket=/ws", "metrics=/metrics"],
                );
                shutdown_channel.1.recv().await;

                Ok(())
            }),
            shutdown_tx: shutdown_channel.0,
        })
    }
}
