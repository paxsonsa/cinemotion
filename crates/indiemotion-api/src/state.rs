use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub struct GlobalState {
    pub controllers: HashMap<String, Controller>,
    pub scene: Scene,
    pub mode: Mode,
}
