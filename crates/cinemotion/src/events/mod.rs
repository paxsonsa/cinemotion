mod connection;
mod error;
mod state;

use crate::messages::Echo;

pub use connection::*;
pub use error::*;
pub use state::*;

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
                EventBody::Error(err) => proto::event::Payload::Error(err.into()),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventBody {
    Echo(Echo),
    ConnectionOpened(ConnectionOpenedEvent),
    StateChanged(StateChangeEvent),
    Error(ErrorEvent),
}

impl From<StateChangeEvent> for EventBody {
    fn from(value: StateChangeEvent) -> Self {
        Self::StateChanged(value)
    }
}

impl From<ErrorEvent> for EventBody {
    fn from(value: ErrorEvent) -> Self {
        Self::Error(value)
    }
}

pub type EventPipeTx = tokio::sync::broadcast::Sender<Event>;
pub type EventPipeRx = tokio::sync::broadcast::Receiver<Event>;

pub fn event_pipe() -> EventPipeTx {
    let (sender, _) = tokio::sync::broadcast::channel(1024);
    sender
}
