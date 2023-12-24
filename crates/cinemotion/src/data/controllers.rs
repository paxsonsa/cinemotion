use crate::Name;
use cinemotion_proto as proto;

use super::PropertyDef;

#[derive(Debug, Clone, PartialEq)]
pub struct Controller {
    pub uid: u32,
    pub name: Name,
    pub properties: Vec<PropertyDef>,
}

impl From<proto::Controller> for Controller {
    fn from(value: proto::Controller) -> Self {
        Self {
            uid: value.uid,
            name: value.name.into(),
            properties: value.properties.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Controller> for proto::Controller {
    fn from(value: Controller) -> Self {
        Self {
            uid: value.uid,
            name: value.name.to_string(),
            properties: value.properties.into_iter().map(Into::into).collect(),
        }
    }
}
