use std::sync::Arc;

use super::Command;
use super::Context;
use super::ContextUpdate;
use super::RuntimeVisitor;

use crate::Result;

pub struct Runtime {
    update_channel: Arc<tokio::sync::broadcast::Sender<ContextUpdate>>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            update_channel: Arc::new(tokio::sync::broadcast::channel(1024).0),
        }
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
        Ok(())
    }

    async fn subscribe(&self) -> Arc<tokio::sync::broadcast::Sender<ContextUpdate>> {
        self.update_channel.clone()
    }
}
