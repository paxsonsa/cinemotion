use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{models::*, Name};

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub struct GlobalState {
    pub controllers: HashMap<Name, Arc<ControllerState>>,
    pub scene: Arc<Scene>,
    pub mode: Mode,
}
