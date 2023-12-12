use async_trait::async_trait;

use crate::commands::EventPipeRx;

#[async_trait]
pub trait SessionAgent {
    async fn initialize(&mut self, response_pipe: EventPipeRx) -> crate::Result<()>;
}
