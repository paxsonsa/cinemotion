use super::value::*;
use crate::{Name, Result};
use cinemotion_proto as proto;

/// Represents a property on a controller.
///
/// A property is the primary way to communcation motion from a controller.
/// When a controller is updated, the property value will be updated and the default value
/// will replace the current value when the motion state is reset.
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    /// The name of the property.
    pub name: Name,
    /// The current value of the property.
    pub value: Value,
    /// The default value of the property to use when the motion state is reset.
    pub default_value: Value,
}

impl Property {
    pub fn with_default_value(name: Name, default_value: Value) -> Self {
        Self {
            name,
            value: default_value.clone(),
            default_value,
        }
    }

    pub fn update(&mut self, value: &Value) -> Result<()> {
        self.value.update(value)
    }

    pub fn reset(&mut self) -> Result<()> {
        self.value.update(&self.default_value)
    }
}

impl From<Property> for proto::Property {
    fn from(value: Property) -> Self {
        Self {
            name: value.name.to_string(),
            value: Some(value.value.into()),
            default_value: Some(value.default_value.into()),
        }
    }
}

impl From<proto::Property> for Property {
    fn from(value: proto::Property) -> Self {
        Self {
            name: value.name.into(),
            value: value.value.unwrap().into(),
            default_value: value.default_value.unwrap().into(),
        }
    }
}

/// A helper struct for representing a property binding address.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyReference {
    /// The namespace of the controller that has the property.
    pub namespace: Name,
    /// The name of the property on the controller to use as
    /// reference for the property's value..
    pub property: Name,
}

/// Represents the property link for a specific property of a scene object.
///
/// The property link can either be unbound, meaning the property not attached
/// to a controller, or bound, meaning the property is attached to a controller property.
///
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyLink {
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
        binding: PropertyReference,
    },
}

impl PropertyLink {
    /// Create new property state bound to the given namespace and property.
    pub fn bind(namespace: Name, property: Name, value: Value) -> Self {
        Self::Bound {
            value,
            binding: PropertyReference {
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

impl From<proto::PropertyLink> for PropertyLink {
    fn from(value: proto::PropertyLink) -> Self {
        match value.bind_state() {
            proto::property_link::BindState::Unbound => Self::unbound(value.value.unwrap().into()),
            proto::property_link::BindState::Bound => Self::bind(
                value.namespace.into(),
                value.property.into(),
                value.value.unwrap().into(),
            ),
        }
    }
}
