use async_trait::async_trait;

use crate::commands::Event;

use super::SendHandlerFn;

#[async_trait]
pub trait SessionAgent: Send + Sync {
    async fn initialize(&mut self, send_fn: SendHandlerFn);
    async fn receive(&mut self, event: Event);
    async fn close(&mut self);
}
