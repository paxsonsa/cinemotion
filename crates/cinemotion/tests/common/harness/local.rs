use super::EngineTestHarness;
use async_trait::async_trait;
use cinemotion::{Engine, EngineState, Event, Request, Result};

pub struct LocalEngineTestHarness {
    engine: Engine,
}

impl LocalEngineTestHarness {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    pub fn boxed(engine: Engine) -> Box<dyn EngineTestHarness> {
        Box::new(Self::new(engine))
    }
}

#[async_trait]
impl EngineTestHarness for LocalEngineTestHarness {
    async fn send_request(&mut self, request: Request) -> Result<()> {
        // implementation goes here
        Ok(())
    }

    async fn observed_events(&self) -> Vec<Event> {
        // implementation goes here
        Vec::new()
    }

    async fn observed_state(&self) -> EngineState {
        // implementation goes here
        EngineState::default()
    }
}
