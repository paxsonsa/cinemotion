use super::Value;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

#[derive(Constructor, Clone, Debug, Serialize, Deserialize)]
pub struct Controller {
    name: String,
    properties: Vec<ControllerProperty>,
}

impl Controller {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn properties(&self) -> &Vec<ControllerProperty> {
        &self.properties
    }

    pub fn properties_mut(&mut self) -> &mut Vec<ControllerProperty> {
        &mut self.properties
    }

    pub fn property(&self, name: &str) -> Option<&ControllerProperty> {
        self.properties.iter().find(|p| p.name() == name)
    }

    pub fn property_mut(&mut self, name: &str) -> Option<&mut ControllerProperty> {
        self.properties.iter_mut().find(|p| p.name() == name)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ControllerProperty {
    name: String,
    default_value: Value,
    value: Value,
}

impl ControllerProperty {
    pub fn new(name: String, default_value: Value) -> Self {
        let value = default_value.clone();
        Self {
            name,
            default_value,
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn default_value(&self) -> &Value {
        &self.default_value
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    pub fn reset(&mut self) {
        self.value = self.default_value.clone();
    }
}
