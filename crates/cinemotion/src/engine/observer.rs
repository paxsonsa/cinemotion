use async_trait::async_trait;

use super::EngineState;
use crate::Event;

#[async_trait]
pub trait Observer: Send + Sync {
    fn on_event(&mut self, event: Event);
    fn on_state(&mut self, state: EngineState);
}
