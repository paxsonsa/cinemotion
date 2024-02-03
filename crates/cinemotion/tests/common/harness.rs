use pretty_assertions_sorted::{assert_eq_sorted, assert_ne};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Mutex;

use cinemotion::{engine, messages, Event, Result, State};

#[derive(Default, Clone)]
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
    fn on_message(&mut self, _: &messages::Message) {}
}

pub struct EngineTestHarness {
    engine: engine::Engine,
    spy: Arc<std::sync::Mutex<ObserverSpy>>,
}

impl EngineTestHarness {
    pub fn new() -> Self {
        let (builder, _) = super::make_engine();

        let spy = Arc::new(std::sync::Mutex::new(ObserverSpy::default()));
        let observer = HarnessObserver { spy: spy.clone() };
        let engine = builder
            .with_engine_observer(Arc::new(Mutex::new(observer)))
            .build()
            .expect("engine should build successfully");
        Self { engine, spy }
    }

    pub fn with_state(state: State) -> Self {
        let (builder, _) = super::make_engine();

        let spy = Arc::new(std::sync::Mutex::new(ObserverSpy::default()));
        let observer = HarnessObserver { spy: spy.clone() };
        let engine = builder
            .with_engine_observer(Arc::new(Mutex::new(observer)))
            .with_inital_state(state)
            .build()
            .expect("engine should build successfully");
        Self { engine, spy }
    }

    pub async fn send_message(&mut self, message: messages::Message) -> Result<()> {
        self.engine.apply(message).await
    }

    pub fn observed_events(&self) -> Vec<Event> {
        let events = self
            .spy
            .lock()
            .expect("lock should not panic")
            .observed_events
            .drain(..)
            .collect();

        events
    }

    pub async fn observed_state(&mut self) -> State {
        let _ = self.engine.tick().await.expect("engine tick should pass.");
        let state = self
            .spy
            .lock()
            .expect("lock should not panic")
            .observed_state
            .clone();
        state
    }
}

#[derive(Debug)]
pub struct Task {
    pub description: String,
    pub action: Action,
}

pub enum Action {
    Message(messages::Message),
    ExpectEvents(Vec<Event>),
    ExpectEvent(Box<dyn FnMut(&Event) -> bool>),
    ExpectState(Box<dyn FnMut(&mut State)>),
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Message(message) => write!(f, "Action::Message({:?})", message),
            Action::ExpectEvents(events) => write!(f, "Action::ExpectEvents({:?})", events),
            Action::ExpectEvent(_) => write!(f, "Action::ExpectEvent(_)"),
            Action::ExpectState(_) => write!(f, "Action::ExpectState(_)"),
        }
    }
}

impl From<messages::Message> for Action {
    fn from(message: messages::Message) -> Self {
        Action::Message(message)
    }
}

#[macro_export]
macro_rules! message {
    ($description:expr, $message:expr) => {
        Task {
            description: $description.to_string(),
            action: Action::Message($message.into()),
        }
    };
}

#[macro_export]
macro_rules! events {
    ($description:expr, $($event:expr),*) => {
        Task {
            description: $description.to_string(),
            action: Action::ExpectEvents(vec![$($event.into()),*]),
        }
    };
}

#[macro_export]
macro_rules! event {
    ($description:expr, $event:expr) => {
        Task {
            description: $description.to_string(),
            action: Action::ExpectEvent(Box::new($event)),
        }
    };
}

#[macro_export]
macro_rules! state {
    ($description:expr, $state:expr) => {
        Task {
            description: $description.to_string(),
            action: Action::ExpectState(Box::new($state)),
        }
    };
}

pub async fn run_harness(harness: &mut EngineTestHarness, tasks: Vec<Task>) {
    for task in tasks {
        println!("⏵ {}", task.description);
        match task.action {
            Action::Message(message) => harness
                .send_message(message)
                .await
                .expect("message should not fail to send"),
            Action::ExpectEvents(expected_events) => {
                let events = harness.observed_events();
                let expected_count = expected_events.len();
                let observed_count = events.len();

                assert!(
                    observed_count >= expected_count,
                    "there are less observed events than expected events"
                );

                let mut start_index = 0;
                for event in events.iter() {
                    if event == &expected_events[0] {
                        break;
                    }
                    start_index += 1;
                }

                assert_ne!(
                    start_index, observed_count,
                    "could not find the start of the expected events"
                );
                assert!(
                    observed_count - start_index >= expected_count,
                    "the remaing events are less than the expected events"
                );

                for (i, expected_event) in expected_events.iter().enumerate() {
                    let index = start_index + i;
                    assert_eq_sorted!(expected_event, &events[index], "the expect event at index {} does not match the observed event at index {}", i, index)
                }
            }
            Action::ExpectEvent(mut event_fn) => {
                let events = harness.observed_events();
                let mut found = false;
                for event in events.iter() {
                    if event_fn(event) {
                        found = true;
                        break;
                    }
                }
                if found {
                    continue;
                }
                panic!("expected event not found",);
            }
            Action::ExpectState(mut state_fn) => {
                let observed_state = harness.observed_state().await;
                let mut expected_state = observed_state.clone();
                state_fn(&mut expected_state);
                assert_eq_sorted!(
                    observed_state,
                    expected_state,
                    "current state does not match expected state:",
                );
            }
        };
    }
}

#[macro_export]
macro_rules! harness {
    ($name:ident, $state:block, $body:block) => {
        paste! {
            #[tokio::test]
            #[tracing_test::traced_test]
            async fn [<test_$name>]() {
                tracing::info!("😍");
                let tasks: Vec<Task> = $body;
                let state = $state;
                let mut harness = EngineTestHarness::with_state(state);
                println!("⏵ running harness with: {:#?}", tasks);
                run_harness(&mut harness, tasks).await;
            }
        }
    };
}
