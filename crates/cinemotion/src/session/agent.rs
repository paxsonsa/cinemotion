use async_trait::async_trait;

use super::SendHandlerFn;

#[async_trait]
pub trait SessionAgent {
    async fn initialize(&mut self, sendFn: SendHandlerFn) -> crate::Result<()>;
}
