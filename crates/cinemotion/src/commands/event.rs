use super::Echo;

#[derive(Clone)]
pub enum EventPayload {
    Echo(Echo),
    //    SessionInit,
}

#[derive(Clone)]
pub struct Event {
    pub target: Option<usize>,
    pub payload: EventPayload,
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
