use super::Component;
use crate::clients;
use crate::Result;
use async_trait::async_trait;
use std::pin::Pin;

pub struct ClientComponent {
    proxy_channel: clients::ProxyCommandsTx,
    future: tokio::task::JoinHandle<std::result::Result<(), crate::Error>>,
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl ClientComponent {
    pub fn builder() -> ClientComponentBuilder {
        ClientComponentBuilder::new()
    }

    pub fn build_proxy(&self) -> crate::clients::ClientService {
        crate::clients::ClientService::new(self.proxy_channel.clone())
    }
}

#[async_trait]
impl Component for ClientComponent {
    fn name(&self) -> &'static str {
        "client"
    }

    async fn shutdown(&self) {
        // the error case here is that we are already stopped because
        // the receiver end of the channel has closed, which is ok
        tracing::info!(name = %self.name(), "shutting down");
        let _ = self.shutdown_tx.send(()).await;
    }
}

impl futures::Future for ClientComponent {
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
pub struct ClientComponentBuilder {
    engine_service: Option<crate::engine::Service>,
}

impl ClientComponentBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn build(self) -> Result<ClientComponent> {
        let shutdown_channel = tokio::sync::mpsc::channel(1);
        let proxy_channel = crate::clients::proxy_channel();

        let mut relay = crate::clients::ClientManager::new(
            self.engine_service.unwrap(),
            proxy_channel.1,
            shutdown_channel.1,
        );

        Ok(ClientComponent {
            proxy_channel: proxy_channel.0,
            future: tokio::task::spawn(async move {
                relay.run().await?;
                Ok(())
            }),
            shutdown_tx: shutdown_channel.0,
        })
    }

    pub fn with_engine_service(mut self, service: crate::engine::Service) -> Self {
        self.engine_service = Some(service);
        self
    }
}
