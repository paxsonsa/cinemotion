use std::pin::Pin;

use async_trait::async_trait;

use super::Service;

pub struct MdnsService {
    future: tokio::task::JoinHandle<std::result::Result<(), crate::Error>>,
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl MdnsService {
    pub fn new(port: u16) -> Self {
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);
        MdnsService {
            future: tokio::task::spawn(async move {
                let responder = libmdns::Responder::new().unwrap();
                let _svc = responder.register(
                    "_http._tcp".to_owned(),
                    "cinemotion".to_owned(),
                    port,
                    &["path=/"],
                );

                shutdown_rx.recv().await;
                Ok(())
            }),
            shutdown_tx,
        }
    }
}

#[async_trait]
impl Service for MdnsService {
    fn name(&self) -> &'static str {
        "mdns"
    }

    async fn shutdown(&self) {
        let _ = self.shutdown_tx.send(()).await;
    }
}

impl futures::Future for MdnsService {
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
