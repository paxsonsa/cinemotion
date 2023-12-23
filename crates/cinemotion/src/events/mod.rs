mod connection;
mod state;

use crate::commands::Echo;

pub use connection::*;
pub use state::StateChangeEvent;

use cinemotion_proto as proto;

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub target: Option<usize>,
    pub body: EventBody,
}

impl Event {
    pub fn new(target: usize, payload: EventBody) -> Self {
        Self {
            target: Some(target),
            body: payload,
        }
    }
}

impl From<Event> for proto::Event {
    fn from(value: Event) -> Self {
        proto::Event {
            payload: Some(match value.body {
                EventBody::Echo(echo) => proto::event::Payload::Echo(echo.into()),
                EventBody::ConnectionOpened(opened) => {
                    proto::event::Payload::ConnectionOpened(opened.into())
                }
                EventBody::StateChanged(change) => {
                    proto::event::Payload::StateChange(change.into())
                }
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventBody {
    Echo(Echo),
    ConnectionOpened(ConnectionOpenedEvent),
    StateChanged(StateChangeEvent),
}

impl From<StateChangeEvent> for EventBody {
    fn from(value: StateChangeEvent) -> Self {
        Self::StateChanged(value)
    }
}
