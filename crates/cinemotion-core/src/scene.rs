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
    pub fn new<N: Into<Name>>(name: N) -> Self {
        Self {
            name: name.into(),
            attributes: HashMap::new(),
        }
    }
    pub fn attributes(&self) -> &HashMap<Name, Attribute> {
        &self.attributes
    }

    pub fn insert_attribute(&mut self, attribute: Attribute) {
        self.attributes.insert(attribute.name().clone(), attribute);
    }
}

pub enum Command {
    AddObject(SceneObject),
    UpdateObject(u32, SceneObject),
    RemoveObject(u32),
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

pub mod commands {

    use super::{Command, Scene, SceneObject};
    use crate::commands::{CommandError, CommandReply, CommandResult};
    use crate::prelude::Name;
    use crate::world::{Entity, World};

    pub fn procces(world: &mut World, command: Command) -> CommandResult {
        match command {
            Command::AddObject(object) => {
                let mut query = world.query::<(&SceneObject, &Name)>();
                for (_, name) in query.iter(&world).collect::<Vec<_>>() {
                    if name == &object.name {
                        let reason = format!("object with name '{}' already exists.", object.name);
                        return Err(CommandError::Failed { reason });
                    }
                }
                let id = add_scene_object(world, object);
                Ok(Some(CommandReply::EntityId(id)))
            }
            Command::UpdateObject(_, _) => todo!(),
            Command::RemoveObject(_) => todo!(),
        }
    }

    pub(super) fn add_scene_object(world: &mut World, object: SceneObject) -> u32 {
        let mut entity = world.spawn(object.name.clone());
        entity.insert(object);
        entity.id().index()
    }
}
