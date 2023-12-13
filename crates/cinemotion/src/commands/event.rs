#[derive(Clone)]
pub enum EventPayload {
    Echo(String),
}

#[derive(Clone)]
pub struct Event {
    pub target: Option<usize>,
    pub payload: EventPayload,
}
