use api::{Property, PropertyValue, ProperyID};
use clap::Command;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

use crate::api;
use crate::{Error, Result};

use super::CommandBuilder;
use super::CommandHandle;
use super::CommandType;
use super::ContextUpdate;

// #[cfg(test)]
// #[path = "./runtime_test.rs"]
// mod runtime_test;

pub struct Handle<CType, CBuilder>
where
    CType: CommandType,
{
    main_loop: tokio::task::JoinHandle<()>,
    cmd_channel: tokio::sync::mpsc::Sender<CommandHandle<CType>>,
    update_channel: Arc<tokio::sync::broadcast::Sender<ContextUpdate>>,
    _phantom: std::marker::PhantomData<CBuilder>,
}

impl<CType, CBuilder> Handle<CType, CBuilder>
where
    CType: CommandType,
    CBuilder: CommandBuilder<Command = CType>,
{
    pub fn new(
        main_loop: tokio::task::JoinHandle<()>,
        cmd_channel: tokio::sync::mpsc::Sender<CommandHandle<CType>>,
        update_channel: Arc<tokio::sync::broadcast::Sender<ContextUpdate>>,
    ) -> Self {
        Self {
            main_loop,
            cmd_channel,
            update_channel,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn connect_as(&self, client: api::ClientMetadata) -> Result<ClientHandle> {
        let update_rx = self.update_channel.subscribe();
        let handle = ClientHandle::new(client.id.clone(), update_rx);

        let (cmd, resp) = CBuilder::new_connect_as(client).await;
        self.cmd_channel.send(cmd).await.unwrap(); //FIXME: handle error

        match resp.await.unwrap() {
            // FIXME Handle Error
            Ok(_) => Ok(handle),
            Err(e) => Err(e),
        }
    }
}

pub struct ClientHandle {
    pub id: Uuid,
    task: tokio::task::JoinHandle<()>,
    pub channels: Option<ClientChannels>,
}

impl ClientHandle {
    pub fn new(id: Uuid, update_rx: tokio::sync::broadcast::Receiver<ContextUpdate>) -> Self {
        let (disconnect_tx, disconnect_rx) = tokio::sync::oneshot::channel();
        let (out_tx, out_rx) = tokio::sync::mpsc::channel(1024);

        let channels = ClientChannels {
            disconnect_tx,
            out_rx,
        };
        let mut update_rx = update_rx;
        let mut disconnect_rx = disconnect_rx;
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = update_rx.recv() => match result {
                        Ok(update) => {
                            tracing::debug!(?id, ?update, "sending update to client");
                            if let Err(_) = out_tx.send(update).await {
                                tracing::error!(?id, "client disconnected");
                                break;
                            }
                        }
                        Err(_) => {
                            tracing::error!(?id, "runtime channel closed, closing client");
                            break;
                        }
                    },
                    _ = &mut disconnect_rx => {
                        tracing::debug!(?id, "client disconnected");
                        break;
                    }
                }
            }
            tracing::debug!(?id, "client task finished")
        });
        Self {
            id,
            task,
            channels: Some(channels),
        }
    }
}

pub struct ClientChannels {
    pub disconnect_tx: tokio::sync::oneshot::Sender<()>,
    pub out_rx: tokio::sync::mpsc::Receiver<ContextUpdate>,
}
