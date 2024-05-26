use crate::attributes::*;
use crate::name::*;
use crate::prelude::*;
use bevy_ecs::prelude::Component;

use std::collections::HashMap;

#[cfg(test)]
#[path = "scene_test.rs"]
mod scene_test;

pub struct Scene {
    name: Name,
}

#[derive(Component)]
pub struct SceneObject {
    name: Name,
    attributes: HashMap<Name, Attribute>,
}

impl SceneObject {
    pub fn new(name: Name) -> Self {
        Self {
            name,
            attributes: HashMap::new(),
        }
    }

    pub fn insert_attribute(&mut self, attribute: Attribute) {}
}

pub mod system {
    use crate::name;
    use crate::name::Name;
    use crate::world::World;

    use super::{Attribute, SceneObject};

    pub fn init(world: &mut World) {
        let name = name!("default");
        let mut entity = world.spawn(name.clone());

        let mut object = SceneObject::new(name);
        object.insert_attribute(Attribute::new_matrix44("transform"));
        entity.insert(object);
    }
}
