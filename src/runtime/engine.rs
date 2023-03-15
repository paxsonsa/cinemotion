use std::sync::Arc;

use super::Command;
use super::Context;
use super::ContextUpdate;
use crate::Result;

#[async_trait::async_trait]
pub trait Engine {
    async fn tick(&mut self, context: &mut Context) -> Result<()>;
    async fn process(&mut self, context: &mut Context, command: Command) -> Result<()>;
    async fn subscribe(&self) -> Arc<tokio::sync::broadcast::Sender<ContextUpdate>>;
}
