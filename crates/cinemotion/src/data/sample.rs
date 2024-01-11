use cinemotion_proto as proto;
use std::collections::HashMap;

use super::Value;
use crate::Name;

#[derive(Debug, PartialEq, Clone)]
pub struct Sample {
    properties: HashMap<Name, Value>,
}

impl Sample {
    pub fn new(properties: HashMap<Name, Value>) -> Self {
        Self { properties }
    }

    pub fn empty() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn properties(&self) -> &HashMap<Name, Value> {
        &self.properties
    }
}

impl From<proto::Sample> for Sample {
    fn from(value: proto::Sample) -> Self {
        Self {
            properties: value
                .properties
                .into_iter()
                .map(|(name, value)| (name.into(), value.into()))
                .collect(),
        }
    }
}
