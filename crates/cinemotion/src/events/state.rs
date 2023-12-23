use cinemotion_proto as proto;

use crate::State;

#[derive(Debug, Clone, PartialEq)]
pub struct StateChangeEvent(pub State);

impl From<StateChangeEvent> for proto::StateChangeEvent {
    fn from(value: StateChangeEvent) -> Self {
        proto::StateChangeEvent {
            state: Some(value.0.into()),
        }
    }
}
