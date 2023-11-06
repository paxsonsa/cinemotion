use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "lowercase")]
pub enum Command {
    Empty,
    Controller(ControllerDef),
    SceneObject(SceneObject),
    Sample(Sample),
    Mode(Mode),
    Disconnect,
}
