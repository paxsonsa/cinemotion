use std::sync::Arc;
use tokio::sync::Mutex;

use cinemotion::*;

mod common;

struct ObserverSpy {
    observed_events: Vec<Event>,
    observed_state: State,
}

struct HarnessObserver {
    pub spy: Arc<std::sync::Mutex<ObserverSpy>>,
}

impl engine::Observer for HarnessObserver {
    fn on_state_change(&mut self, new_state: &State) {
        self.spy.lock().unwrap().observed_state = new_state.clone();
    }
    fn on_event(&mut self, event: &Event) {
        self.spy.lock().unwrap().observed_events.push(event.clone());
    }
    fn on_request(&mut self, request: &Request) {}
}

struct EngineTestHarness {
    engine: Engine,
    spy: Arc<std::sync::Mutex<ObserverSpy>>,
}

impl EngineTestHarness {
    fn new() -> Self {
        let (builder, _) = common::make_engine();

        let spy = Arc::new(std::sync::Mutex::new(ObserverSpy {
            observed_events: Vec::new(),
            observed_state: State::default(),
        }));
        let observer = HarnessObserver { spy: spy.clone() };
        let engine = builder
            .with_engine_observer(Arc::new(Mutex::new(observer)))
            .build()
            .expect("engine should build successfully");
        Self { engine, spy }
    }

    async fn send_request(&mut self, request: Request) -> Result<()> {
        self.engine.apply(request).await
    }

    fn observed_events(&self) -> Vec<Event> {
        let events = self
            .spy
            .lock()
            .expect("lock should not panic")
            .observed_events
            .clone();
        events
    }

    fn observed_state(&self) -> State {
        let state = self
            .spy
            .lock()
            .expect("lock should not panic")
            .observed_state
            .clone();
        state
    }
}

struct Task {
    pub description: String,
    pub action: Action,
}

enum Action {
    Request(Request),
    ExpectEvents(Vec<Event>),
    ExpectState(State),
}

async fn run_harness(harness: &mut EngineTestHarness, tasks: Vec<Task>) {
    for task in tasks {
        println!("{}", task.description);
        match task.action {
            Action::Request(request) => harness
                .send_request(request)
                .await
                .expect("request should not fail to send"),
            Action::ExpectEvents(expected_events) => {
                let events = harness.observed_events();
                let expected_count = expected_events.len();
                let observed_count = events.len();

                assert!(observed_count >= expected_count);

                let mut start_index = 0;
                for event in events.iter() {
                    if event == &expected_events[0] {
                        break;
                    }
                    start_index += 1;
                }

                assert_ne!(start_index, observed_count);
                assert!(observed_count - start_index >= expected_count);

                for (i, expected_event) in expected_events.iter().enumerate() {
                    let index = start_index + i;
                    assert_eq!(expected_event, &events[index]);
                }
            }
            Action::ExpectState(expected_state) => {
                let observed_state = harness.observed_state();
                assert_eq!(observed_state, expected_state);
            }
        };
    }
}

#[tokio::test]
async fn test_session_initialize() {
    let mut harness = EngineTestHarness::new();
    let tasks = vec![];
    run_harness(&mut harness, tasks).await;
}
