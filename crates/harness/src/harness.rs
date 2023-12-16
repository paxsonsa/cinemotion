use async_trait::async_trait;
use cinemotion::*;

#[async_trait]
pub trait EngineTestHarness {
    async fn send_command(&mut self, command: Command) -> Result<()>;
    async fn observe_event(&self) -> Vec<Event>;
    async fn observe_state(&self) -> EngineState;
}
