use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Value;
use crate::Name;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sample {
    properties: HashMap<Name, Value>,
}

impl Sample {
    pub fn new(properties: HashMap<Name, Value>) -> Self {
        Self { properties }
    }

    pub fn properties(&self) -> &HashMap<Name, Value> {
        &self.properties
    }
}
