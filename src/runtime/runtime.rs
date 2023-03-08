use std::sync::Arc;

use super::Context;
use super::ContextUpdate;
use super::RuntimeVisitor;
use super::{CommandBuilder, CommandHandle, CommandResult, CommandType};

use crate::{api, Result};

#[derive(Debug, Clone)]
pub enum DefaultCommand {
    Ping,
    ConnectAs(api::ClientMetadata),
}

impl CommandType for DefaultCommand {}

impl std::fmt::Display for DefaultCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefaultCommand::Ping => write!(f, "Ping()"),
            DefaultCommand::ConnectAs(client) => write!(f, "ConnectAs({:?})", client),
        }
    }
}

#[async_trait::async_trait]
impl CommandBuilder for DefaultCommand {
    type Command = DefaultCommand;

    async fn new_ping() -> (CommandHandle<DefaultCommand>, CommandResult) {
        CommandHandle::new(DefaultCommand::Ping)
    }

    async fn new_connect_as(
        client: api::ClientMetadata,
    ) -> (CommandHandle<DefaultCommand>, CommandResult) {
        CommandHandle::new(DefaultCommand::ConnectAs(client))
    }
}

pub struct DefaultRuntime {
    update_channel: Arc<tokio::sync::broadcast::Sender<ContextUpdate>>,
}

impl DefaultRuntime {
    fn new() -> Self {
        Self {
            update_channel: Arc::new(tokio::sync::broadcast::channel(1024).0),
        }
    }
}

impl Default for DefaultRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl RuntimeVisitor<DefaultCommand> for DefaultRuntime {
    async fn visit_command(
        &mut self,
        context: &mut Context,
        command: DefaultCommand,
    ) -> Result<()> {
        tracing::info!("Incoming command: {:?}", command);
        Ok(())
    }

    async fn subscribe(&self) -> Arc<tokio::sync::broadcast::Sender<ContextUpdate>> {
        self.update_channel.clone()
    }
}
