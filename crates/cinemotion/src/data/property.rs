use serde::{Deserialize, Serialize};

use super::value::*;
use crate::Name;
use cinemotion_proto as proto;

/// A generic defintion for a property.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDef {
    /// The name of the property.
    name: Name,

    /// The default value to use when idle and the type the property is.
    default_value: Value,
}

impl PropertyDef {
    /// Build a new property definition.
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

impl From<proto::PropertyDef> for PropertyDef {
    fn from(value: proto::PropertyDef) -> Self {
        Self {
            name: value.name.into(),
            default_value: value.default_value.unwrap().into(),
        }
    }
}

impl From<PropertyDef> for proto::PropertyDef {
    fn from(value: PropertyDef) -> Self {
        Self {
            name: value.name.to_string(),
            default_value: Some(value.default_value.into()),
        }
    }
}

/// A helper struct for representing a property binding address.
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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

    pub fn unbound(value: Value) -> Self {
        Self::Unbound { value }
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

impl From<proto::PropertyState> for PropertyState {
    fn from(value: proto::PropertyState) -> Self {
        match value.r#type() {
            proto::property_state::StateType::Unbound => Self::unbound(value.value.unwrap().into()),
            proto::property_state::StateType::Bound => Self::bind(
                value.namespace.into(),
                value.property.into(),
                value.value.unwrap().into(),
            ),
        }
    }
}
