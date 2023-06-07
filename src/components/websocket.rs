use clap::Args;
use futures::{FutureExt, SinkExt, StreamExt, TryFutureExt};
use std::pin::Pin;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{ws::WebSocket, Filter};

use crate::async_trait;
use crate::server::Component;
use crate::Result;

#[derive(Default, Debug, Clone, Args)]
pub struct WebsocketServiceBuilder {
    /// The local socket address on which to serve the grpc endpoint
    #[clap(long = "server.bind-address")]
    server_bind_address: Option<std::net::SocketAddr>,
}

impl WebsocketServiceBuilder {
    pub async fn build(self) -> Result<WebsocketService> {
        let addr = self
            .server_bind_address
            .unwrap_or_else(|| ([0, 0, 0, 0], crate::DEFAULT_WEB_PORT).into());

        let router = warp::path("connect")
            .and(warp::ws())
            // .and(warp::any().map(move || gateway.clone()))
            .map(|ws: warp::ws::Ws| ws.on_upgrade(move |socket| connect(socket)));

        let server = warp::serve(router);
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);
        let future = tokio::spawn(async move {
            tokio::select! {
                _ = server.run(addr) => {
                    tracing::info!("websocket service finished");
                    Ok(())
                },
                _ = shutdown_rx.recv() => {
                    tracing::info!("websocket service shutdown");
                    Ok(())
                }
            }
        });

        Ok(WebsocketService {
            future,
            shutdown_tx,
        })
    }
}

pub struct WebsocketService {
    future: tokio::task::JoinHandle<std::result::Result<(), crate::Error>>,
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl WebsocketService {}

#[async_trait]
impl Component for WebsocketService {
    fn name(&self) -> &'static str {
        "websocket"
    }

    async fn shutdown(&self) {
        // the error case here is that we are already stopped because
        // the receiver end of the channel has closed, which is ok
        let _ = self.shutdown_tx.send(()).await;
    }
}

impl futures::Future for WebsocketService {
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

async fn connect(ws: WebSocket) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    let handle = tokio::spawn(async move {
        while let Some(message) = rx.next().await {
            ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    tracing::error!("websocket send error: {}", e);
                })
                .await;
        }
    });

    while let Some(result) = ws_rx.next().await {
        let message = match result {
            Ok(message) => message,
            Err(e) => {
                tracing::error!("websocket error: {}", e);
                break;
            }
        };

        tracing::debug!("received message: {:?}", message);
    }
}
