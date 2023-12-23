use cinemotion_proto as proto;

use crate::engine::State;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateChangeEvent(pub State);

impl From<StateChangeEvent> for proto::StateChangeEvent {
    fn from(value: StateChangeEvent) -> Self {
        proto::StateChangeEvent {
            state: Some(value.0.into()),
        }
    }
}
