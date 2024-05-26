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

#[derive(Component, Clone)]
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
            Command::UpdateObject(id, object) => match set_scene_object(world, id, object) {
                Some(id) => Ok(Some(CommandReply::EntityId(id))),
                None => Err(CommandError::NotFound),
            },
            Command::RemoveObject(id) => match remove_scene_object_by_id(world, id) {
                Some(id) => Ok(Some(CommandReply::EntityId(id))),
                None => Err(CommandError::NotFound),
            },
        }
    }

    pub(super) fn add_scene_object(world: &mut World, object: SceneObject) -> u32 {
        let mut entity = world.spawn(object.name.clone());
        entity.insert(object);
        entity.id().index()
    }

    pub(super) fn set_scene_object(world: &mut World, id: u32, object: SceneObject) -> Option<u32> {
        let entity = Entity::from_raw(id);
        let Some(mut entity) = world.get_entity_mut(entity) else {
            return None;
        };
        entity.insert(object);
        Some(entity.id().index())
    }

    pub(super) fn remove_scene_object_by_id(world: &mut World, device_id: u32) -> Option<u32> {
        let entity = Entity::from_raw(device_id);
        match world.despawn(entity) {
            true => Some(device_id),
            false => None,
        }
    }
}
