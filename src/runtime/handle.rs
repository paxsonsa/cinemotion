use api::{Property, PropertyValue, ProperyID};
use std::fmt::Debug;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use uuid::Uuid;

use crate::api;
use crate::{Error, Result};

use super::Command;
use super::CommandHandle;
use super::ContextUpdate;

// #[cfg(test)]
// #[path = "./runtime_test.rs"]
// mod runtime_test;

pub struct Handle {
    main_loop: tokio::task::JoinHandle<()>,
    cmd_channel: tokio::sync::mpsc::Sender<CommandHandle>,
    update_channel: Arc<tokio::sync::broadcast::Sender<ContextUpdate>>,
}

impl Handle {
    async fn send(&self, command: Command) -> Result<tokio::sync::oneshot::Receiver<Result<()>>> {
        let (cmd, resp) = CommandHandle::new(command);
        self.cmd_channel.send(cmd).await.unwrap(); //FIXME: handle error

        Ok(resp)
    }

    pub fn new(
        main_loop: tokio::task::JoinHandle<()>,
        cmd_channel: tokio::sync::mpsc::Sender<CommandHandle>,
        update_channel: Arc<tokio::sync::broadcast::Sender<ContextUpdate>>,
    ) -> Self {
        Self {
            main_loop,
            cmd_channel,
            update_channel,
        }
    }

    pub async fn ping(&self) -> Result<i64> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let resp = self.send(Command::Ping(tx)).await.unwrap();

        if let Err(_) = resp.await {
            // TODO: Handle error
            return Err(Error::InternalError("Failed to ping"));
        }
        Ok(rx.await.unwrap())
    }

    pub async fn connect_as(&self, client: api::ClientMetadata) -> Result<ClientHandle> {
        let update_rx = self.update_channel.subscribe();
        let handle = ClientHandle::new(client.id.clone(), self.cmd_channel.clone(), update_rx);

        let (cmd, resp) = CommandHandle::new(Command::ConnectAs(client));
        self.cmd_channel.send(cmd).await.unwrap(); //FIXME: handle error

        match resp.await.unwrap() {
            // FIXME Handle Error
            Ok(_) => Ok(handle),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub struct ClientHandle {
    pub id: Uuid,
    pub inner: tokio_stream::wrappers::BroadcastStream<ContextUpdate>,
    pub cmd_tx: tokio::sync::mpsc::Sender<CommandHandle>,
}

impl ClientHandle {
    pub fn new(
        id: Uuid,
        cmd_tx: tokio::sync::mpsc::Sender<CommandHandle>,
        update_rx: tokio::sync::broadcast::Receiver<ContextUpdate>,
    ) -> Self {
        let stream = tokio_stream::wrappers::BroadcastStream::new(update_rx);
        // let task = tokio::spawn(async move {
        //     loop {
        //         tokio::select! {
        //             result = update_rx.recv() => match result {
        //                 Ok(update) => {
        //                     tracing::debug!(?id, ?update, "sending update to client");
        //                     if let Err(_) = out_tx.send(update).await {
        //                         tracing::error!(?id, "client disconnected");
        //                         break;
        //                     }
        //                 }
        //                 Err(_) => {
        //                     tracing::error!(?id, "runtime channel closed, closing client");
        //                     break;
        //                 }
        //             }
        //         }
        //     }
        //     tracing::debug!(?id, "client task finished")
        // });

        Self {
            id,
            inner: stream,
            cmd_tx,
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
        tracing::warn!("Client has disconnected: {}", self.id);
        // TODO: handle disconnect;
        // let (cmd, _) = CommandHandle::new(Command::Disconnect(self.id));
        // self.cmd_tx.blocking_send(cmd);
    }
}
