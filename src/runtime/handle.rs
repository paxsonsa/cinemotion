use api::{Property, PropertyValue, ProperyID};
use std::fmt::Debug;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use tracing_futures::Instrument;

use crate::api;
use crate::{Error, Result};

use super::Command;
use super::CommandHandle;
use super::Context;
use super::ContextUpdate;
use super::Engine;

// #[cfg(test)]
// #[path = "./runtime_test.rs"]
// mod runtime_test;

pub struct Handle {
    _main_loop: tokio::task::JoinHandle<()>,
    cmd_channel: tokio::sync::mpsc::Sender<CommandHandle>,
    update_channel: Arc<tokio::sync::broadcast::Sender<ContextUpdate>>,
}

impl Handle {
    async fn send(&self, command: Command) -> Result<tokio::sync::oneshot::Receiver<Result<()>>> {
        let (cmd, resp) = CommandHandle::new(command);
        if let Err(_) = self.cmd_channel.send(cmd).await {
            return Err(Error::RuntimeError(
                "failed to send command to runtime. channel closed",
            ));
        }

        Ok(resp)
    }

    pub async fn new<Visitor>(
        mut visitor: Box<Visitor>,
        mut shutdown_rx: tokio::sync::mpsc::Receiver<()>,
    ) -> Self
    where
        Visitor: Engine + Send + 'static,
    {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<CommandHandle>(1024);

        let update_channel = visitor.subscribe().await;
        let main_loop = tokio::spawn(async move {
            let mut context = Context::default();
            loop {
                tokio::select! {

                    // Listen for shutdown signal
                    _ = shutdown_rx.recv() => {
                        tracing::info!("runtime received shutdown signal");
                        break;
                    }

                    // Listen for new commands
                    item = rx.recv() => match item {
                        Some(handle) => {
                            tracing::debug!("received command: {:?}", handle);

                            let (command, reply) = handle.decompose();

                            match visitor.process(&mut context, command).instrument(tracing::trace_span!("runtime")).await {
                                Ok(_) => {
                                    tracing::debug!("command processed successfully");
                                    if let Err(err) = reply.send(Ok(())) {
                                        tracing::error!("reply channel closed while sending reply: {:?}", err);
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("error while processing command: {:?}", e);
                                    if let Err(err) = reply.send(Err(e)) {
                                        tracing::error!("reply channel closed while sending reply: {:?}", err);
                                    }
                                }
                            };
                        }
                        None => {
                            tracing::info!("command channel closed");
                            break;
                        }
                    }
                }
            }
        });

        Self {
            _main_loop: main_loop,
            cmd_channel: tx,
            update_channel,
        }
    }

    /// Send a ping command to the runtime and return the current timestamp
    pub async fn ping(&self) -> Result<i64> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let resp = self.send(Command::Ping(tx)).await.unwrap();

        if let Err(_) = resp.await {
            return Err(Error::InternalError("Failed to ping"));
        }
        Ok(rx.await.unwrap())
    }

    /// Connect a client to the runtime and return a handle to the client.
    pub async fn connect_as(&self, client: api::ClientMetadata) -> Result<ClientHandle> {
        let update_rx = self.update_channel.subscribe();
        let mut handle = ClientHandle::new(client.id.clone(), self.cmd_channel.clone(), update_rx);
        let resp = self.send(Command::ConnectAs(client)).await?;

        match resp.await.unwrap() {
            Ok(_) => Ok(handle),
            Err(err) => match err {
                Error::ClientError(_) => {
                    handle.close_on_drop = false;
                    Err(err)
                }
                _ => Err(err),
            },
        }
    }
}

#[derive(Debug)]
pub struct ClientHandle {
    pub id: api::ClientID,
    pub inner: tokio_stream::wrappers::BroadcastStream<ContextUpdate>,
    pub cmd_tx: tokio::sync::mpsc::Sender<CommandHandle>,
    pub close_on_drop: bool,
}

impl ClientHandle {
    pub fn new(
        id: api::ClientID,
        cmd_tx: tokio::sync::mpsc::Sender<CommandHandle>,
        update_rx: tokio::sync::broadcast::Receiver<ContextUpdate>,
    ) -> Self {
        tracing::warn!("Client handle has connect: {}", id);
        let stream = tokio_stream::wrappers::BroadcastStream::new(update_rx);

        Self {
            id,
            inner: stream,
            cmd_tx,
            close_on_drop: true,
        }
    }
}

impl tokio_stream::Stream for ClientHandle {
    type Item = std::result::Result<
        ContextUpdate,
        tokio_stream::wrappers::errors::BroadcastStreamRecvError,
    >;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl Deref for ClientHandle {
    type Target = tokio_stream::wrappers::BroadcastStream<ContextUpdate>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Drop for ClientHandle {
    fn drop(&mut self) {
        if self.close_on_drop {
            tracing::warn!("Client handle has disconnected: {}", self.id);
            let _ = self
                .cmd_tx
                .try_send(CommandHandle::new(Command::Disconnect(self.id.clone())).0);
        }
    }
}
