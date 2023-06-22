use serde::{Deserialize, Serialize};

use crate::Name;

use super::value::*;
/// A generic defintion for a property.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyDef {
    /// The name of the property.
    pub name: Name,

    /// The default value to use when idle and the type the property is.
    pub default_value: Value,
}

/// A helper struct for representing a property binding address.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyBinding {
    /// The namespace of the controller that has the property.
    pub namespace: Name,
    /// The name of the property on the controller to use as
    /// reference for the property's value..
    pub property: Name,
}

/// Represents the property state for a specific property of a scene object.
///
/// The property state can either be unbound, meaning the property not attached
/// to a controller, or bound, meaning the property is attached to a controller property.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PropertyState {
    /// An unbound property does not reference a controller property for updates.
    Unbound {
        /// The current value of the property.
        value: Value,
    },

    /// A bound property references a controller property for updates.
    Bound {
        /// The current value of the property.
        value: Value,

        /// The binding for the property to use.
        binding: PropertyBinding,
    },
}

impl From<Value> for PropertyState {
    fn from(value: Value) -> Self {
        Self::Unbound { value }
    }
}

impl PropertyState {
    /// Create new property state bound to the given namespace and property.
    pub fn bind(namespace: Name, property: Name, value: Value) -> Self {
        Self::Bound {
            value,
            binding: PropertyBinding {
                namespace,
                property,
            },
        }
    }

    /// Return the underlying value regardless of the state.
    pub fn value(&self) -> &Value {
        match self {
            Self::Unbound { value } => value,
            Self::Bound { value, .. } => value,
        }
    }

    /// Return whether the property is bound to a controller property.
    pub fn has_binding(&self) -> bool {
        match self {
            Self::Unbound { .. } => false,
            Self::Bound { .. } => true,
        }
    }
}
