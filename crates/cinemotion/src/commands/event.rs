use super::ConnectionOpened;
use super::Echo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventPayload {
    Echo(Echo),
    ConnectionOpened(ConnectionOpened),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub target: Option<usize>,
    pub payload: EventPayload,
}

impl Event {
    pub fn new(target: usize, payload: EventPayload) -> Self {
        Self {
            target: Some(target),
            payload,
        }
    }
}
impl From<Event> for cinemotion_proto::Event {
    fn from(value: Event) -> Self {
        cinemotion_proto::Event {
            payload: Some(match value.payload {
                EventPayload::Echo(echo) => cinemotion_proto::event::Payload::Echo(echo.into()),
                EventPayload::ConnectionOpened(hello) => {
                    cinemotion_proto::event::Payload::ConnectionOpened(hello.into())
                }
            }),
        }
    }
}
