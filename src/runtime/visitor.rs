use std::sync::Arc;

use super::Command;
use super::{Context, ContextUpdate};
use crate::Result;

#[async_trait::async_trait]
pub trait RuntimeVisitor {
    async fn visit_command(&mut self, context: &mut Context, command: Command) -> Result<()>;
    async fn subscribe(&self) -> Arc<tokio::sync::broadcast::Sender<ContextUpdate>>;
}
