use futures::future::Future;
use paste::paste;
use std::sync::Arc;
use std::{pin::Pin, usize};
use tokio::sync::Mutex;

use cinemotion::{
    commands, connection::LOCAL_CONN_ID, data, engine, events, Event, EventBody, Message, Result,
};

mod common;

#[derive(Default, Clone)]
struct ObserverSpy {
    observed_events: Vec<Event>,
    observed_state: engine::State,
}

struct HarnessObserver {
    pub spy: Arc<std::sync::Mutex<ObserverSpy>>,
}

impl engine::Observer for HarnessObserver {
    fn on_state_change(&mut self, new_state: &engine::State) {
        self.spy.lock().unwrap().observed_state = new_state.clone();
    }
    fn on_event(&mut self, event: &Event) {
        self.spy.lock().unwrap().observed_events.push(event.clone());
    }
    fn on_request(&mut self, request: &Message) {}
}

struct EngineTestHarness {
    engine: engine::Engine,
    spy: Arc<std::sync::Mutex<ObserverSpy>>,
}

impl EngineTestHarness {
    fn new() -> Self {
        let (builder, _) = common::make_engine();

        let spy = Arc::new(std::sync::Mutex::new(ObserverSpy::default()));
        let observer = HarnessObserver { spy: spy.clone() };
        let engine = builder
            .with_engine_observer(Arc::new(Mutex::new(observer)))
            .build()
            .expect("engine should build successfully");
        Self { engine, spy }
    }

    fn with_state(state: engine::State) -> Self {
        let (builder, _) = common::make_engine();

        let spy = Arc::new(std::sync::Mutex::new(ObserverSpy::default()));
        let observer = HarnessObserver { spy: spy.clone() };
        let engine = builder
            .with_engine_observer(Arc::new(Mutex::new(observer)))
            .with_inital_state(state)
            .build()
            .expect("engine should build successfully");
        Self { engine, spy }
    }

    async fn send_request(&mut self, request: Message) -> Result<()> {
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

    async fn observed_state(&mut self) -> engine::State {
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

struct Task {
    pub description: String,
    pub action: Action,
    pub assertion: Option<Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>>,
}

enum Action {
    Message(Message),
    ExpectEvents(Vec<Event>),
    ExpectEvent(Box<dyn FnMut(&Event) -> bool>),
    ExpectState(Box<dyn FnMut(&mut engine::State)>),
}

impl From<Message> for Action {
    fn from(request: Message) -> Self {
        Action::Message(request)
    }
}

macro_rules! request {
    ($description:expr, $request:expr) => {
        Task {
            description: $description.to_string(),
            action: Action::Message($request.into()),
            assertion: None,
        }
    };
}

macro_rules! events {
    ($description:expr, $($event:expr),*) => {
        Task {
            description: $description.to_string(),
            action: Action::ExpectEvents(vec![$($event.into()),*]),
            assertion: None,
        }
    };
}

macro_rules! event {
    ($description:expr, $event:expr) => {
        Task {
            description: $description.to_string(),
            action: Action::ExpectEvent(Box::new($event)),
            assertion: None,
        }
    };
}

macro_rules! state {
    ($description:expr, $state:expr) => {
        Task {
            description: $description.to_string(),
            action: Action::ExpectState(Box::new($state)),
            assertion: None,
        }
    };
}

async fn run_harness(harness: &mut EngineTestHarness, tasks: Vec<Task>) {
    for task in tasks {
        println!("⏵ {}", task.description);
        match task.action {
            Action::Message(request) => harness
                .send_request(request)
                .await
                .expect("request should not fail to send"),
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
                    assert_eq!(expected_event, &events[index], "the expect event at index {} does not match the observed event at index {}", i, index)
                }
            }
            Action::ExpectEvent(mut event_fn) => {
                let events = harness.observed_events();
                for event in events.iter() {
                    if event_fn(event) {
                        return;
                    }
                }
                panic!("expected event not found",);
            }
            Action::ExpectState(mut state_fn) => {
                let observed_state = harness.observed_state().await;
                let mut expected_state = observed_state.clone();
                state_fn(&mut expected_state);
                assert_eq!(
                    observed_state, expected_state,
                    "current state does not match expected state:\ncurrent: {:#?}\n!=\n expect: {:#?}",
                    observed_state, expected_state
                );
            }
        };
    }
}

macro_rules! harness {
    ($name:ident, $state:block, $body:block) => {
        paste! {
            #[tokio::test]
            async fn [<test_$name>]() {
                let tasks = $body;
                let state = $state;
                let mut harness = EngineTestHarness::with_state(state);
                run_harness(&mut harness, tasks).await;
            }
        }
    };
}

harness!(connection_setup, { engine::State::default() }, {
    let source_id: usize = 1;
    let (ack_pipe, _ack_pipe_rx) = tokio::sync::oneshot::channel();

    vec![
        request!(
            "create connection",
            Message {
                source_id: LOCAL_CONN_ID,
                command: commands::AddConnection {
                    agent: Box::<common::session::DummyAgent>::default(),
                    ack_pipe,
                }
                .into(),
            }
        ),
        request!(
            "open connection",
            Message {
                source_id, // Hardcoded Id that should be set.
                command: commands::OpenConnection {}.into(),
            }
        ),
        events!(
            "expect hello event to be sent",
            Event {
                target: Some(source_id),
                body: events::ConnectionOpenedEvent {}.into(),
            }
        ),
        request!(
            "initial connection session",
            Message {
                source_id,
                command: commands::Init {
                    peer: data::Peer {
                        name: "test".to_string(),
                        role: data::PeerRole::Controller,
                    }
                }
                .into(),
            }
        ),
        state!(
            "expect the peer information to be in the public state",
            |state: &mut engine::State| {
                state.peers = vec![data::Peer {
                    name: "test".to_string(),
                    role: data::PeerRole::Controller,
                }];
            }
        ),
        event!("expect some state event to be emitted", |event: &Event| {
            matches!(event.body, EventBody::StateChanged(_))
        }),
    ]
});

harness!(
    peer_mapping,
    {
        let mut state = engine::State::default();
        state.peers = vec![data::Peer {
            name: "test".to_string(),
            role: data::PeerRole::Controller,
        }];
        state
    },
    { vec![] }
);
