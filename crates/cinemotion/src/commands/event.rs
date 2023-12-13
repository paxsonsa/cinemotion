#[derive(Clone)]
pub enum EventPayload {
    Echo(String),
    SessionInit,
}

#[derive(Clone)]
pub struct Event {
    pub target: Option<usize>,
    pub payload: EventPayload,
}
