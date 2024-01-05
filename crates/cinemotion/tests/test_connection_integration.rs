use paste::paste;
use std::collections::HashMap;

use cinemotion::{
    commands, connection::LOCAL_CONN_ID, data, events, name, Event, EventBody, Message, State,
};

mod common;
use common::*;

harness!(connection_setup, { State::default() }, {
    let (ack_pipe, _ack_pipe_rx) = tokio::sync::oneshot::channel();

    vec![
        message!(
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
        message!(
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
        message!(
            "initial connection session",
            Message {
                source_id: 1,
                command: commands::Init {
                    peer: data::Controller {
                        uid: 1,
                        name: name!("test"),
                        properties: vec![data::Property::with_default_value(
                            name!("position"),
                            data::Value::Vec3((0.0, 0.0, 0.0).into()),
                        )]
                        .into_iter()
                        .map(|p| (p.name.clone(), p))
                        .collect(),
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
                        properties: vec![data::Property::with_default_value(
                            name!("position"),
                            data::Value::Vec3((0.0, 0.0, 0.0).into()),
                        )]
                        .into_iter()
                        .map(|p| (p.name.clone(), p))
                        .collect(),
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
