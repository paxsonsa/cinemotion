use std::sync::Arc;
use std::collections::VecDeque;

use rstest::rstest;

use crate::api;
use crate::controller::Controller;

#[rstest]
#[tokio::test]
async fn test_tick_response() {
    let tick = api::Tick {
        metadata: api::Metadata {
            source: "client".to_string(),
            destination: None,
            source_time_ms: 100,
        },
        spec: api::TickSpec {},
    };

    let mut controller= super::ServiceController::new();
    let result = controller.update(api::TimeSpec::new(10, 105), tick.into()).await.expect("update should not fail.");

    match result {
        super::ControllerUpdateResult::RespondStop(response) => {
            match response {
                api::Message::Tock(tock) => {
                    assert_eq!(tock.spec.time, 100);
                },
                _ => panic!("Unexpected response type."),
            }
        },
        _ => panic!("Unexpected result type."),
    }
}