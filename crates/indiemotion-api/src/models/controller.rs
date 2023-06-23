use super::{PropertyDef, Value};
use crate::Name;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
#[path = "./controller_test.rs"]
mod controller_test;

/// Represents the current state of a controller in the system.
///
/// The state contains the definition metadata used to create the controller
/// and the current values for each defined property.
///
/// Property values are reset to their default values when the system is idle.
///
#[derive(Constructor, Debug, Serialize, Deserialize, Clone)]
pub struct ControllerState {
    /// The current values of the controller properties.
    ///
    /// When the system is live/recording, these values are updated by the engine.
    ///
    values: HashMap<Name, Value>,

    /// The definition of the controller and its properties.
    metadata: ControllerDef,
}

/// A controller state can be created from a controller definition.
impl From<ControllerDef> for ControllerState {
    fn from(definition: ControllerDef) -> Self {
        Self {
            values: definition
                .properties()
                .iter()
                .map(|p| (p.name().clone(), p.default_value().clone()))
                .collect(),
            metadata: definition,
        }
    }
}

impl ControllerState {
    /// Return the name of the controller as defined in the metadata.
    pub fn name(&self) -> &Name {
        self.metadata.name()
    }

    /// Given a controller definition, update the current defintion and properties.
    ///
    /// All current values are reset to their default values and added/removed.
    ///
    pub fn redefine(&mut self, definition: ControllerDef) {
        self.values = definition
            .properties()
            .iter()
            .map(|p| (p.name().clone(), p.default_value().clone()))
            .collect();
        self.metadata = definition;
    }

    /// Reset all property values to their default values.
    pub fn reset(&mut self) {
        self.metadata.properties().iter().for_each(|p| {
            if let Some(value) = self.values.get_mut(p.name()) {
                *value = p.default_value().clone();
            }
        });
    }

    /// Return a reference to the current values of the controllers properties.
    pub fn values(&self) -> &HashMap<Name, Value> {
        &self.values
    }

    /// Return a mutable reference to the current values of the controllers properties.
    pub fn values_mut(&mut self) -> &mut HashMap<Name, Value> {
        &mut self.values
    }

    /// Return a reference to a specific property value.
    ///
    /// `None` is returned if the property does not exist.
    ///
    pub fn value(&self, name: &Name) -> Option<&Value> {
        self.values.get(name)
    }

    /// Return a mutable reference to a specific property value.
    ///
    /// /// `None` is returned if the property does not exist.
    ///
    pub fn value_mut(&mut self, name: &Name) -> Option<&mut Value> {
        self.values.get_mut(name)
    }
}

/// A definition of a controller to be added to the system.
#[derive(Constructor, Clone, Debug, Serialize, Deserialize)]
pub struct ControllerDef {
    /// A unique name for the controller.
    name: Name,

    /// A list of properties on the controller that will be recorded.
    properties: Vec<PropertyDef>,
}

impl ControllerDef {
    /// Return the name of the controller.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Return the properties on the controller.
    pub fn properties(&self) -> &Vec<PropertyDef> {
        &self.properties
    }

    /// Return mutable properties on the controller.
    pub fn properties_mut(&mut self) -> &mut Vec<PropertyDef> {
        &mut self.properties
    }

    /// Return a property on the controller.
    ///
    /// `None` is returned if the property does not exist.
    ///
    pub fn property(&self, name: &Name) -> Option<&PropertyDef> {
        self.properties.iter().find(|p| p.name() == name)
    }

    /// Return a mutable reference to a property on the controller.
    ///
    /// `None` is returned if the property does not exist.
    ///
    pub fn property_mut(&mut self, name: &Name) -> Option<&mut PropertyDef> {
        self.properties.iter_mut().find(|p| p.name() == name)
    }
}

/// A definition of a property on a controller.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ControllerPropertyDef {
    /// A unique name for the property on the controller.
    name: Name,

    /// A default value the property. This both defines the type of the property
    /// and the default value when resetting the controller.
    default_value: Value,
}

impl ControllerPropertyDef {
    /// Build a new controller property definition.
    pub fn new(name: Name, default_value: Value) -> Self {
        Self {
            name,
            default_value,
        }
    }

    /// Get the name of the property.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Get the default value of the property.
    pub fn default_value(&self) -> &Value {
        &self.default_value
    }
}
