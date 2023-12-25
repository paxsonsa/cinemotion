use cinemotion::data::PropertyState;
use futures::future::Future;
use paste::paste;
use pretty_assertions_sorted::{assert_eq, assert_eq_sorted, assert_ne};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::{pin::Pin, usize};
use tokio::sync::Mutex;

use cinemotion::{
    commands, connection::LOCAL_CONN_ID, data, engine, events, name, scene, Error, Event,
    EventBody, Message, Result, State,
};

mod common;

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

    fn with_state(state: State) -> Self {
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
            .drain(..)
            .collect();

        events
    }

    async fn observed_state(&mut self) -> State {
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
struct Task {
    pub description: String,
    pub action: Action,
}

enum Action {
    Message(Message),
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
        }
    };
}

macro_rules! events {
    ($description:expr, $($event:expr),*) => {
        Task {
            description: $description.to_string(),
            action: Action::ExpectEvents(vec![$($event.into()),*]),
        }
    };
}

macro_rules! event {
    ($description:expr, $event:expr) => {
        Task {
            description: $description.to_string(),
            action: Action::ExpectEvent(Box::new($event)),
        }
    };
}

macro_rules! state {
    ($description:expr, $state:expr) => {
        Task {
            description: $description.to_string(),
            action: Action::ExpectState(Box::new($state)),
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

macro_rules! harness {
    ($name:ident, $state:block, $body:block) => {
        paste! {
            #[tokio::test]
            async fn [<test_$name>]() {
                let tasks: Vec<Task> = $body;
                let state = $state;
                let mut harness = EngineTestHarness::with_state(state);
                println!("⏵ running harness with: {:#?}", tasks);
                run_harness(&mut harness, tasks).await;
            }
        }
    };
}

harness!(connection_setup, { State::default() }, {
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
                source_id: 1, // Hardcoded Id that should be set.
                command: commands::OpenConnection {}.into(),
            }
        ),
        events!(
            "expect hello event to be sent",
            Event {
                target: Some(1),
                body: events::ConnectionOpenedEvent().into(),
            }
        ),
        request!(
            "initial connection session",
            Message {
                source_id: 1,
                command: commands::Init {
                    peer: data::Controller {
                        uid: 1,
                        name: name!("test"),
                        properties: vec![data::PropertyDef::new(
                            name!("position"),
                            data::Value::Vec3((0.0, 0.0, 0.0).into()),
                        )],
                    }
                }
                .into(),
            }
        ),
        state!(
            "expect the peer information to be in the public state",
            |state: &mut State| {
                let mut controllers = HashMap::new();
                controllers.insert(
                    name!("test"),
                    data::Controller {
                        uid: 1,
                        name: name!("test"),
                        properties: vec![data::PropertyDef::new(
                            name!("position"),
                            data::Value::Vec3((0.0, 0.0, 0.0).into()),
                        )],
                    },
                );
                state.controllers = controllers;
            }
        ),
        event!("expect some state event to be emitted", |event: &Event| {
            matches!(event.body, EventBody::StateChanged(_))
        }),
    ]
});

harness!(
    scene_object_commands,
    {
        let mut state = State::default();
        let mut controllers = HashMap::new();
        controllers.insert(
            name!("test"),
            data::Controller {
                uid: 1,
                name: name!("test"),
                properties: vec![data::PropertyDef::new(
                    name!("position"),
                    data::Value::Vec3((0.0, 0.0, 0.0).into()),
                )],
            },
        );
        state.controllers = controllers;
        state
    },
    {
        vec![
            request!(
                "attempt to update a scene object that does not exist",
                Message {
                    source_id: 1,
                    command: commands::UpdateSceneObject(scene::SceneObject::new(
                        name!("doesnotexist"),
                        HashMap::from([(
                            name!("position"),
                            data::PropertyState::bind(
                                name!("test"),
                                name!("position"),
                                data::Value::Vec3((0.0, 0.0, 0.0).into())
                            ),
                        )])
                    ))
                    .into(),
                }
            ),
            event!("expect an error event to be emitted", |event: &Event| {
                match event.target {
                    Some(1) => match &event.body {
                        EventBody::Error(event) => {
                            matches!(event.0, Error::InvalidSceneObject(_))
                        }
                        _ => false,
                    },

                    _ => false,
                }
            }),
            request!(
                "add a new scene object.",
                Message {
                    source_id: 1,
                    command: commands::AddSceneObject(scene::SceneObject::new(
                        name!("object1"),
                        HashMap::from([(
                            name!("position"),
                            PropertyState::unbound(data::Vec3::from((0.0, 0.0, 0.0)).into()),
                        )])
                    ))
                    .into(),
                }
            ),
            state!(
                "verify that the scene object is in the state",
                |state: &mut State| {
                    state.scene.objects_mut().insert(
                        name!("object1"),
                        scene::SceneObject::new(
                            name!("object1"),
                            HashMap::from([(
                                name!("position"),
                                PropertyState::unbound(data::Vec3::from((0.0, 0.0, 0.0)).into()),
                            )]),
                        ),
                    );
                }
            ),
            request!(
                "try to add a existing scene object.",
                Message {
                    source_id: 1,
                    command: commands::AddSceneObject(scene::SceneObject::new(
                        name!("object1"),
                        HashMap::from([(
                            name!("position"),
                            PropertyState::unbound(data::Vec3::from((0.0, 0.0, 0.0)).into()),
                        )])
                    ))
                    .into(),
                }
            ),
            event!("expect an error event to be emitted", |event: &Event| {
                match event.target {
                    Some(1) => match &event.body {
                        EventBody::Error(event) => {
                            matches!(event.0, Error::InvalidSceneObject(_))
                        }
                        _ => false,
                    },

                    _ => false,
                }
            }),
            request!(
                "update the root scene object to map controller property to object",
                Message {
                    source_id: 1,
                    command: commands::UpdateSceneObject(scene::SceneObject::new(
                        name!("default"),
                        HashMap::from([(
                            name!("position"),
                            data::PropertyState::bind(
                                name!("test"),
                                name!("position"),
                                data::Value::Vec3((0.0, 0.0, 0.0).into())
                            ),
                        )])
                    ))
                    .into(),
                }
            ),
            state!(
                "verify that the scene objects property is bound to the controller",
                |state: &mut State| {
                    state
                        .scene
                        .objects_mut()
                        .get_mut(&name!("default"))
                        .expect("expected default object")
                        .properties_mut()
                        .insert(
                            name!("position"),
                            data::PropertyState::bind(
                                name!("test"),
                                name!("position"),
                                data::Value::Vec3((0.0, 0.0, 0.0).into()),
                            ),
                        );
                }
            ),
            request!(
                "delete object in the scene",
                Message {
                    source_id: 1,
                    command: commands::DeleteSceneObject(name!("object1"),).into(),
                }
            ),
            state!("check that the object was deleted", |state: &mut State| {
                let _ = state.scene.objects_mut().remove(&name!("object1"));
            }),
        ]
    }
);
