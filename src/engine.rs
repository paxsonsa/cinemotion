use crate::runtime::Command;
use crate::runtime::Context;
use crate::runtime::Integrator;
use crate::{api, Error, Result};

pub struct Engine {}

impl Engine {
    fn new() -> Self {
        Self {}
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
        ctx.clients.insert(client.id.clone(), client);
        let _clients = ctx.clients.values().cloned().collect::<Vec<_>>();
        tracing::debug!("ctx: {:?}", ctx);
        Ok(())
    }

    async fn handle_disconnect(&mut self, ctx: &mut Context, id: api::ClientID) -> Result<()> {
        tracing::warn!("disconnecting client: {}", id);
        // TODO: ensure mode is updated.
        ctx.clients.remove(&id);
        let _clients = ctx.clients.values().cloned().collect::<Vec<_>>();
        tracing::debug!("ctx: {:?}", ctx);

        Ok(())
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Integrator for Engine {
    async fn tick(&mut self, _context: &mut Context) -> Result<()> {
        Ok(())
    }
    async fn process(&mut self, context: &mut Context, command: Command) -> Result<()> {
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
}
