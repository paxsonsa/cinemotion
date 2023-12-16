use super::harness::EngineTestHarness;
use async_trait::async_trait;
use cinemotion::*;

pub struct LocalEngineTestHarness {
    engine: Engine,
}

impl LocalEngineTestHarness {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }
}

#[async_trait]
impl EngineTestHarness for LocalEngineTestHarness {
    async fn send_command(&mut self, command: Command) -> Result<()> {
        // implementation goes here
        Ok(())
    }

    async fn observe_event(&self) -> Vec<Event> {
        // implementation goes here
        Vec::new()
    }

    async fn observe_state(&self) -> EngineState {
        // implementation goes here
        EngineState::default()
    }
}
