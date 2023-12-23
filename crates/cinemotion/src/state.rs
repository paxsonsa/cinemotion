use std::collections::HashMap;

use cinemotion_proto as proto;

use crate::data::*;
use crate::Scene;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct State {
    pub scene: Scene,
    pub peers: HashMap<u32, Peer>,
}

impl From<State> for proto::State {
    fn from(value: State) -> Self {
        proto::State {
            peers: value.peers.into_values().map(Into::into).collect(),
        }
    }
}
