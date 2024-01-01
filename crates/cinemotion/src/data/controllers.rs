use crate::Name;
use cinemotion_proto as proto;
use std::collections::HashMap;

use super::Property;

/// Represents a controller in the system.
///
/// A controller is a source of motion in the system. It can be used to control
/// the motion of a scene object by binding a property to a controller property.
#[derive(Debug, Clone, PartialEq)]
pub struct Controller {
    /// The unique identifier of the controller.
    pub uid: u32,
    /// The name of the controller used for users.
    pub name: Name,
    /// The properties of the controller that hold motion state.
    pub properties: HashMap<Name, Property>,
}

impl From<proto::ControllerDef> for Controller {
    fn from(value: proto::ControllerDef) -> Self {
        Self {
            uid: value.uid,
            name: value.name.into(),
            properties: value
                .properties
                .into_iter()
                .map(|(name, value)| {
                    let property = Property::with_default_value(name.clone().into(), value.into());
                    (name.into(), property)
                })
                .collect(),
        }
    }
}

impl From<proto::Controller> for Controller {
    fn from(value: proto::Controller) -> Self {
        Self {
            uid: value.uid,
            name: value.name.into(),
            properties: value
                .properties
                .into_iter()
                .map(|(name, property)| (name.into(), property.into()))
                .collect(),
        }
    }
}

impl From<Controller> for proto::Controller {
    fn from(value: Controller) -> Self {
        Self {
            uid: value.uid,
            name: value.name.to_string(),
            properties: value
                .properties
                .into_iter()
                .map(|(name, property)| (name.to_string(), property.into()))
                .collect(),
        }
    }
}
