mod local;
mod step;

use async_trait::async_trait;
use cinemotion::*;
pub use local::LocalEngineTestHarness;
pub use step::Step;

#[async_trait]
pub trait EngineTestHarness: Send + Sync {
    async fn send_request(&mut self, request: Request) -> Result<()>;
    async fn observed_events(&self) -> Vec<Event>;
    async fn observed_state(&self) -> EngineState;

    async fn execute(&mut self, steps: Vec<Step>) -> Result<()> {
        for step in steps {
            match step {
                Step::Request(request) => self.send_request(request).await?,
                Step::Event(event) => {
                    let observed_events = self.observed_events().await;
                    assert!(observed_events.contains(&event));
                }
                Step::State(state) => {
                    let observed_state = self.observed_state().await;
                    assert_eq!(observed_state, state);
                }
            }
        }
        Ok(())
    }
}

pub enum HarnessKind {
    Local,
}

pub async fn run_harness(kind: HarnessKind, steps: Vec<Step>) -> Result<()> {
    let mut harness = match kind {
        HarnessKind::Local => LocalEngineTestHarness::boxed(engine),
    };
    harness.execute(steps).await
}
