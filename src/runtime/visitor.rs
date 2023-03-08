use std::sync::Arc;

use super::{Context, ContextUpdate};
use crate::Result;

#[async_trait::async_trait]
pub trait RuntimeVisitor<CommandType> {
    async fn visit_command(&mut self, context: &mut Context, command: CommandType) -> Result<()>;
    async fn subscribe(&self) -> Arc<tokio::sync::broadcast::Sender<ContextUpdate>>;
}
