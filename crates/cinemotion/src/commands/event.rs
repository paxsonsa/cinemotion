use super::Echo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventPayload {
    Echo(Echo),
    //    SessionInit,
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

impl Into<cinemotion_proto::Event> for Event {
    fn into(self) -> cinemotion_proto::Event {
        cinemotion_proto::Event {
            payload: Some(match self.payload {
                EventPayload::Echo(echo) => cinemotion_proto::event::Payload::Echo(echo.into()),
            }),
        }
    }
}
