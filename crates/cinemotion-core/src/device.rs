use std::collections::HashMap;

use crate::error::*;
use crate::prelude::*;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Device {
    name: Name,
    attributes: HashMap<Name, Attribute>,
}

impl Device {
    pub fn new(name: Name) -> Self {
        Self {
            name,
            attributes: HashMap::new(),
        }
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }

    pub fn attributes(&self) -> &HashMap<Name, Attribute> {
        &self.attributes
    }

    pub fn insert_attribute(&mut self, attribute: Attribute) -> Option<Attribute> {
        self.attributes.insert(attribute.name().clone(), attribute)
    }
}
