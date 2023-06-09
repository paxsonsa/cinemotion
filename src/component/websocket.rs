use clap::Args;
use futures::{SinkExt, StreamExt};
use std::pin::Pin;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};

use super::Component;
use crate::api;
use crate::clients::{Client, ClientRelayProxy};
use crate::Result;
use async_trait::async_trait;

pub struct WebsocketComponent {
    future: tokio::task::JoinHandle<std::result::Result<(), crate::Error>>,
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl WebsocketComponent {
    pub fn builder() -> WebsocketComponentBuilder {
        WebsocketComponentBuilder::default()
    }
}

#[async_trait]
impl Component for WebsocketComponent {
    fn name(&self) -> &'static str {
        "websocket"
    }

    async fn shutdown(&self) {
        // the error case here is that we are already stopped because
        // the receiver end of the channel has closed, which is ok
        let _ = self.shutdown_tx.send(()).await;
    }
}

impl futures::Future for WebsocketComponent {
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
pub struct WebsocketComponentBuilder {
    /// The local socket address on which to serve the grpc endpoint
    server_bind_address: Option<std::net::SocketAddr>,

    /// The client proxy to use when working with clients.
    client_proxy: Option<ClientRelayProxy>,
}

impl WebsocketComponentBuilder {
    pub async fn build(self) -> Result<WebsocketComponent> {
        let addr = self
            .server_bind_address
            .unwrap_or_else(|| ([0, 0, 0, 0], crate::DEFAULT_WEB_PORT).into());

        let Some(client_proxy) = self.client_proxy else {
            todo!()
        };

        let router = warp::path("connect")
            .and(warp::ws())
            .and(warp::any().map(move || client_proxy.clone()))
            .map(|ws: warp::ws::Ws, proxy: ClientRelayProxy| {
                ws.on_upgrade(move |socket| connect(socket, proxy))
            });

        let server = warp::serve(router);
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);
        let future = tokio::spawn(async move {
            tracing::info!("started webscoket service {:?}", addr);
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

        Ok(WebsocketComponent {
            future,
            shutdown_tx,
        })
    }

    pub fn with_server_bind_address(mut self, addr: std::net::SocketAddr) -> Self {
        self.server_bind_address = Some(addr);
        self
    }

    pub fn with_client_proxy(mut self, client_proxy: ClientRelayProxy) -> Self {
        self.client_proxy = Some(client_proxy);
        self
    }
}

async fn connect(ws: WebSocket, client_relay: ClientRelayProxy) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut update_channel = tokio::sync::mpsc::unbounded_channel();

    let client = Client::new(update_channel.0.clone());
    let handle = match client_relay.register_client(client).await {
        Ok(handle) => handle,
        Err(e) => {
            tracing::error!("client registration error: {}", e);
            // TODO: Send Error to Client
            return;
        }
    };

    loop {
        tokio::select! {
            Some(msg) = ws_rx.next() => {
                // TODO Handle bad message processing.
                let _ = match msg {
                    Ok(msg) => {
                        tracing::info!("websocket msg: {:?}", msg);

                        let message = match msg.to_str() {
                            Ok(v) => v,
                            Err(_) => return,
                        };

                        match api::command::Encoder::<api::command::JSONProtocol>::decode(message) {
                            Ok(command) => client_relay.receive_from(handle, command).await,
                            Err(e) => {
                                tracing::error!("message decoding error error: {}", e);
                                todo!("add error conversion in JSON");
                            }
                        }
                    },
                    Err(e) => {
                        tracing::error!("websocket error: {}", e);
                        break;
                    }
                };
            },
            Some(update) = update_channel.1.recv() => {

                match ws_tx.send(Message::text(update)).await {
                    Ok(_) => (),
                    Err(e) => {
                        tracing::error!("websocket error: {}", e);
                        break;
                    }
                }

                // match api::command::Encoder::<api::command::JSONProtocol>::encode(update) {
                //     Ok(update) => ws_tx.send(update).await,
                //     Err(e) => {
                //         tracing::error!("message decoding error error: {}", e);
                //         todo!("add error conversion in JSON");
                //     }
                // }
            }
        }
    }
}
