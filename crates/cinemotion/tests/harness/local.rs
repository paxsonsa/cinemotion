use super::EngineTestHarness;
use async_trait::async_trait;
use cinemotion::{Engine, EngineState, Event, Request, Result};

pub struct LocalEngineTestHarness {
    engine: Engine,
    spy: super::common::SpySessionComponent,
}

impl LocalEngineTestHarness {
    pub fn new() -> Self {
        let (engine, spy) = super::common::make_engine();

        Self { engine, spy }
    }

    pub fn boxed() -> Box<dyn EngineTestHarness> {
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
