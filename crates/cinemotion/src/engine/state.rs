use cinemotion_proto as proto;

use crate::data::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct State {
    pub peers: Vec<Peer>,
}

impl From<State> for proto::State {
    fn from(value: State) -> Self {
        proto::State {
            peers: value.peers.into_iter().map(Into::into).collect(),
        }
    }
}
