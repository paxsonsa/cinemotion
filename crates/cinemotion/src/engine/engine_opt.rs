use crate::commands::{EventPipeTx, RequestPipeTx};

pub struct EngineOpt {
    pub request_pipe: RequestPipeTx,
    pub event_pipe: EventPipeTx,
}
