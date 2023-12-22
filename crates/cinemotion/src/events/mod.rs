mod connection;

use crate::commands::Echo;

pub use connection::*;

use cinemotion_proto as proto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventBody {
    Echo(Echo),
    ConnectionOpened(ConnectionOpened),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub target: Option<usize>,
    pub payload: EventBody,
}

impl Event {
    pub fn new(target: usize, payload: EventBody) -> Self {
        Self {
            target: Some(target),
            payload,
        }
    }
}
impl From<Event> for proto::Event {
    fn from(value: Event) -> Self {
        proto::Event {
            payload: Some(match value.payload {
                EventBody::Echo(echo) => proto::event::Payload::Echo(echo.into()),
                EventBody::ConnectionOpened(opened) => {
                    proto::event::Payload::ConnectionOpened(opened.into())
                }
            }),
        }
    }
}
