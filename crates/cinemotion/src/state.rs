use std::collections::HashMap;

use cinemotion_proto as proto;

use crate::data::*;
use crate::Name;
use crate::Scene;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct State {
    pub scene: Scene,
    pub controllers: HashMap<Name, Controller>,
}

impl From<State> for proto::State {
    fn from(value: State) -> Self {
        proto::State {
            controllers: value.controllers.into_values().map(Into::into).collect(),
        }
    }
}
