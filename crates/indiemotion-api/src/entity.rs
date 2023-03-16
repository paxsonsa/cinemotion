use indiemotion_proto as proto;
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq)]
pub struct EntityID(u32);

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: EntityID,
    pub attrs: HashMap<crate::AttributeID, crate::Attribute>,
    pub components: HashMap<crate::ComponentID, crate::Component>,
}

impl Into<proto::Entity> for Entity {
    fn into(self) -> proto::Entity {
        proto::Entity {}
    }
}

impl Into<proto::Entity> for &Entity {
    fn into(self) -> proto::Entity {
        proto::Entity {}
    }
}
