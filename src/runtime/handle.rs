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
use super::Event;
use super::Integrator;

pub struct Handle {
    _main_loop: tokio::task::JoinHandle<Result<()>>,
    cmd_channel: tokio::sync::mpsc::Sender<CommandHandle>,
    event_chan: Arc<tokio::sync::broadcast::Sender<Event>>,
}

impl Handle {
    async fn send(&self, command: Command) -> Result<tokio::sync::oneshot::Receiver<Result<()>>> {
        let (cmd, resp) = CommandHandle::new(command);
        if (self.cmd_channel.send(cmd).await).is_err() {
            return Err(Error::RuntimeError(
                "failed to send command to runtime. channel closed",
            ));
        }

        Ok(resp)
    }

    pub async fn new<E>(
        mut engine: Box<E>,
        mut shutdown_rx: tokio::sync::mpsc::Receiver<()>,
    ) -> Self
    where
        E: Integrator + Send + 'static,
    {
        let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::channel::<CommandHandle>(1024);
        let event_tx = Arc::new(tokio::sync::broadcast::channel(1024).0);
        let event_chan = event_tx.clone();
        let mut interval = tokio::time::interval(std::time::Duration::from_micros(16666));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        let main_loop = tokio::spawn(async move {
            let mut context = Context::default();
            loop {
                tokio::select! {
                    // Listen for engine tick
                    _ = interval.tick() => {
                        engine.tick(&mut context).instrument(tracing::trace_span!("engine")).await?;

                        // Ok to ignore the result here as the broadcast channel might not have
                        // any subscribers but the engine should still tick.
                        let _ = event_tx.send(Event::Context(context.clone()));
                    }

                    // Listen for shutdown signal
                    _ = shutdown_rx.recv() => {
                        tracing::info!("runtime received shutdown signal");
                        break;
                    }

                    // Listen for new commands
                    item = cmd_rx.recv() => match item {
                        Some(handle) => {
                            tracing::debug!("received command: {:?}", handle);

                            let (command, reply) = handle.decompose();

                            match engine.process(&mut context, command).instrument(tracing::trace_span!("runtime")).await {
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
            Ok(())
        });

        Self {
            _main_loop: main_loop,
            cmd_channel: cmd_tx,
            event_chan,
        }
    }

    /// Send a ping command to the runtime and return the current timestamp
    pub async fn ping(&self) -> Result<i64> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let resp = self.send(Command::Ping(tx)).await.unwrap();

        if (resp.await).is_err() {
            return Err(Error::InternalError("Failed to ping"));
        }
        Ok(rx.await.unwrap())
    }

    /// Connect a client to the runtime and return a handle to the client.
    pub async fn connect_as(&self, client: api::ClientMetadata) -> Result<ClientHandle> {
        let event_rx = self.event_chan.subscribe();
        let mut handle = ClientHandle::new(client.id.clone(), self.cmd_channel.clone(), event_rx);
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
    pub inner: tokio_stream::wrappers::BroadcastStream<Event>,
    pub cmd_tx: tokio::sync::mpsc::Sender<CommandHandle>,
    pub close_on_drop: bool,
}

impl ClientHandle {
    pub fn new(
        id: api::ClientID,
        cmd_tx: tokio::sync::mpsc::Sender<CommandHandle>,
        update_rx: tokio::sync::broadcast::Receiver<Event>,
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
    type Item =
        std::result::Result<Event, tokio_stream::wrappers::errors::BroadcastStreamRecvError>;

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
    type Target = tokio_stream::wrappers::BroadcastStream<Event>;

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
