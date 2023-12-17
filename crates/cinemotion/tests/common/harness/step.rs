use cinemotion::{EngineState, Event, Request};

pub enum Step {
    Request(Request),
    Event(Event),
    State(EngineState),
}

impl From<Request> for Step {
    fn from(request: Request) -> Self {
        Self::Request(request)
    }
}

impl From<Event> for Step {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}

impl From<EngineState> for Step {
    fn from(state: EngineState) -> Self {
        Self::State(state)
    }
}
