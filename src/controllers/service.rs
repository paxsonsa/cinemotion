use api::HasMetadata;

use crate::controller::{Controller, ControllerUpdateResult};
use crate::api;
use crate::Result;

#[cfg(test)]
#[path = "./service_test.rs"]
mod service_test;

pub struct ServiceController {}

impl ServiceController {
    pub fn new() -> Self {
        ServiceController {}
    }

    pub fn new_boxed() -> Box<Self> {
        Box::new(ServiceController::new())
    }
}

#[crate::async_trait]
impl Controller for ServiceController {
    fn name() -> &'static str {
        "service"
    }

    async fn update(&self, time: api::TimeSpec, message: api::Message) -> Result<ControllerUpdateResult> {

        match message {
            api::Message::Tick(tick) => {
                let response = api::Message::Tock(api::Tock {
                    metadata: api::Metadata {
                        source: "service".to_string(),
                        destination: Some(tick.metadata().source),
                        source_time_ms: time.time_ms(),
                    },
                    spec: api::TockSpec {
                        time: tick.metadata().source_time_ms,
                    },
                });

                Ok(ControllerUpdateResult::RespondStop(response))
            },
            _ => Ok(ControllerUpdateResult::Next),
        }
    }
}