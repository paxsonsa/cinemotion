use std::{net::SocketAddr, pin::Pin};

use async_trait::async_trait;
use warp::{self, Filter};

use super::Service;

pub struct HttpService {
    future: tokio::task::JoinHandle<std::result::Result<(), crate::Error>>,
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl HttpService {
    pub fn new<I>(address: I) -> Self
    where
        I: Into<SocketAddr> + Send + 'static,
    {
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);

        let root = warp::path::end().map(|| "CineMotion Server");
        let service = warp::serve(root).run(address);

        HttpService {
            future: tokio::task::spawn(async move {
                tokio::select! {
                    _ = service => {}
                    _ = shutdown_rx.recv() => {}
                }
                Ok(())
            }),
            shutdown_tx,
        }
    }
}

#[async_trait]
impl Service for HttpService {
    fn name(&self) -> &'static str {
        "http"
    }

    async fn shutdown(&self) {
        let _ = self.shutdown_tx.send(()).await;
    }
}

impl futures::Future for HttpService {
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
