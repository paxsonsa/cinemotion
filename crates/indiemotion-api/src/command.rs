use serde_derive::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "lowercase")]
pub enum Command {
    Empty,
    Controller(Controller),
    SceneObject(SceneObject),
    Sample(Sample),
    Mode(Mode),
}
