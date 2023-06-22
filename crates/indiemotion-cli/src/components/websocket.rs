use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use std::pin::Pin;
use warp::{ws, ws::WebSocket, Filter};

use super::Component;
use crate::api;
use crate::clients::{Client, ClientService};
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
    client_proxy: Option<ClientService>,
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
            .map(|ws: warp::ws::Ws, proxy: ClientService| {
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

    pub fn with_client_proxy(mut self, client_proxy: ClientService) -> Self {
        self.client_proxy = Some(client_proxy);
        self
    }
}

async fn connect(ws: WebSocket, client_service: ClientService) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut message_channel = tokio::sync::mpsc::unbounded_channel();

    let client = Client::new(message_channel.0.clone());
    let handle = match client_service.connect(client).await {
        Ok(handle) => handle,
        Err(err) => {
            tracing::error!("client registration failed: {}", err);
            handle_error(
                api::Error::UnexpectedError(format!("client registration failed {:#}", err)),
                &mut ws_tx,
            )
            .await;
            return;
        }
    };
    tracing::info!("client connected: {}", handle);

    loop {
        tokio::select! {
            Some(msg) = ws_rx.next() => {
                let msg = match msg {
                    Ok(m) => m,
                    Err(err) => {
                        tracing::error!("websocket error: {}", err);
                        let _ = client_service.disconnect(handle).await;
                        break;
                    }
                };

                if msg.is_close() {
                    let _ = client_service.disconnect(handle).await;
                    break;
                }

                if let Err(err) = handle_message(handle, &msg, &client_service).await {
                    handle_error(err, &mut ws_tx).await;
                }
            },
            Some(message) = message_channel.1.recv() => {
                let msg =
                    api::message::Encoding::<api::message::JSONProtocol>::encode(&message)
                    .unwrap();
                match ws_tx.send(ws::Message::text(msg)).await {
                    Ok(_) => (),
                    Err(e) => {
                        tracing::error!("websocket error: {}", e);
                        let _ = client_service.disconnect(handle).await;
                        break;
                    }
                }
            }
        }
    }
    tracing::info!("closing connection: {}", handle);
}

async fn handle_error(err: api::Error, websocket: &mut SplitSink<WebSocket, ws::Message>) {
    let msg =
        api::message::Encoding::<api::message::JSONProtocol>::encode(&api::Message::Error(err))
            .unwrap();
    match websocket.send(ws::Message::text(msg)).await {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("websocket error: {}", e);
        }
    }
}

async fn handle_message(
    handle: u32,
    msg: &ws::Message,
    client_relay: &ClientService,
) -> api::Result<()> {
    tracing::info!("websocket msg: {:?}", msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return Ok(()),
    };

    match api::message::Encoding::<api::message::JSONProtocol>::decode(message) {
        Ok(msg) => {
            let api::Message::Command(command) = msg else {
                        tracing::info!("client {} sent invalid message type.", handle);
                        return Err(api::Error::BadMessage("clients can only send command type messages".to_string()));
                    };

            if let Err(err) = client_relay.send_from(handle, command).await {
                tracing::error!(
                    "client {} command processing failed: {}, message: {}",
                    handle,
                    err,
                    message
                );
                return Err(api::Error::BadMessage(
                    "clients can only send command type messages".to_string(),
                ));
            }
            Ok(())
        }
        Err(err) => {
            tracing::error!(?err, message, "message decoding error occured");
            Err(err)
        }
    }
}