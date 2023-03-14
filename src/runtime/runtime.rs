use std::sync::Arc;

use super::Command;
use super::Context;
use super::ContextUpdate;
use super::RuntimeVisitor;
use crate::{api, Error, Result};

pub struct Runtime {
    update_channel: Arc<tokio::sync::broadcast::Sender<ContextUpdate>>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            update_channel: Arc::new(tokio::sync::broadcast::channel(1024).0),
        }
    }

    async fn handle_connect(
        &mut self,
        ctx: &mut Context,
        client: api::ClientMetadata,
    ) -> Result<()> {
        if let Some(client) = ctx.clients.get(&client.id) {
            tracing::error!("client already connected: {}", client.id.clone());
            return Err(Error::ClientError("client already connected".to_string()));
        }
        ctx.clients.insert(client.id.clone(), client.clone());

        let clients = ctx
            .clients
            .values()
            .map(|client| client.clone())
            .collect::<Vec<_>>();

        self.update_channel
            .send(ContextUpdate::Client(clients))
            .unwrap();

        Ok(())
    }

    async fn handle_disconnect(&mut self, ctx: &mut Context, id: api::ClientID) -> Result<()> {
        tracing::info!("connecting client: {}", id);
        // TODO: ensure mode is updated.
        ctx.clients.remove(&id);

        let clients = ctx
            .clients
            .values()
            .map(|client| client.clone())
            .collect::<Vec<_>>();

        self.update_channel
            .send(ContextUpdate::Client(clients))
            .unwrap();
        Ok(())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl RuntimeVisitor for Runtime {
    async fn visit_command(&mut self, context: &mut Context, command: Command) -> Result<()> {
        tracing::info!("Incoming command: {:?}", command);

        match command {
            Command::Ping(tx) => {
                let _ = tx.send(chrono::Utc::now().timestamp_millis());
                Ok(())
            }
            Command::ConnectAs(client) => self.handle_connect(context, client).await,
            Command::Disconnect(client_id) => self.handle_disconnect(context, client_id).await,
        }
    }

    async fn subscribe(&self) -> Arc<tokio::sync::broadcast::Sender<ContextUpdate>> {
        self.update_channel.clone()
    }
}
