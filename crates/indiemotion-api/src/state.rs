use serde_derive::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub struct GlobalState {
    pub clients: Vec<Client>,
    pub scene: SceneGraph,
}
