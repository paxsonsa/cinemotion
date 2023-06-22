use super::Value;
use crate::Name;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Constructor, Debug, Serialize, Deserialize, Clone)]
pub struct ControllerState {
    values: HashMap<Name, Value>,
    definition: ControllerDef,
}

impl From<ControllerDef> for ControllerState {
    fn from(definition: ControllerDef) -> Self {
        Self {
            values: definition
                .properties()
                .iter()
                .map(|p| (p.name().clone(), p.default_value().clone()))
                .collect(),
            definition,
        }
    }
}

impl ControllerState {
    pub fn name(&self) -> &Name {
        self.definition.name()
    }

    pub fn redefine(&mut self, definition: ControllerDef) {
        self.values = definition
            .properties()
            .iter()
            .map(|p| (p.name().clone(), p.default_value().clone()))
            .collect();
        self.definition = definition;
    }

    pub fn reset(&mut self) {
        self.definition.properties().iter().for_each(|p| {
            if let Some(value) = self.values.get_mut(&p.name()) {
                *value = p.default_value().clone();
            }
        });
    }

    pub fn values(&self) -> &HashMap<Name, Value> {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut HashMap<Name, Value> {
        &mut self.values
    }

    pub fn value(&self, name: &Name) -> Option<&Value> {
        self.values.get(name)
    }

    pub fn value_mut(&mut self, name: &Name) -> Option<&mut Value> {
        self.values.get_mut(name)
    }
}

#[derive(Constructor, Clone, Debug, Serialize, Deserialize)]
pub struct ControllerDef {
    name: Name,
    properties: Vec<ControllerPropertyDef>,
}

impl ControllerDef {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn properties(&self) -> &Vec<ControllerPropertyDef> {
        &self.properties
    }

    pub fn properties_mut(&mut self) -> &mut Vec<ControllerPropertyDef> {
        &mut self.properties
    }

    pub fn property(&self, name: &Name) -> Option<&ControllerPropertyDef> {
        self.properties.iter().find(|p| p.name() == name)
    }

    pub fn property_mut(&mut self, name: &Name) -> Option<&mut ControllerPropertyDef> {
        self.properties.iter_mut().find(|p| p.name() == name)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ControllerPropertyDef {
    name: Name,
    default_value: Value,
}

impl ControllerPropertyDef {
    pub fn new(name: String, default_value: Value) -> Self {
        Self {
            name: name.into(),
            default_value,
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn default_value(&self) -> &Value {
        &self.default_value
    }
}

#[derive(Constructor, Clone, Debug, Serialize, Deserialize)]
pub struct ControllerPropertyState {
    name: String,
    value: Value,
}
