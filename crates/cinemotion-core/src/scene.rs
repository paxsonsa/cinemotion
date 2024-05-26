use crate::attributes::*;
use crate::prelude::*;

use std::collections::HashMap;

#[cfg(test)]
#[path = "scene_test.rs"]
mod scene_test;

pub struct Scene {
    name: Name,
}

#[derive(Clone)]
pub struct SceneObject {
    pub name: Name,
    pub attributes: HashMap<Name, Attribute>,
}

impl SceneObject {
    pub fn new<N: Into<Name>>(name: N) -> Self {
        Self {
            name: name.into(),
            attributes: HashMap::new(),
        }
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
    use crate::name::*;
    use crate::world::World;

    use bevy_ecs::prelude::{Bundle, Component, Entity};

    use super::{Attribute, AttributeLinkMap, AttributeMap, SceneObject};

    #[derive(Component, Debug)]
    pub struct SceneObjectEntity;

    pub struct SceneObjectEntityRef<'w> {
        entity: Entity,
        world: &'w World,
    }

    impl SceneObjectEntityRef<'_> {
        pub fn name(&self) -> Name {
            self.world.get::<Name>(self.entity).unwrap().clone()
        }

        pub fn attributes(&self) -> &AttributeMap {
            self.world.get::<AttributeMap>(self.entity).unwrap()
        }
    }

    pub fn init(world: &mut World) {
        let mut object = SceneObject::new("default");
        object.insert_attribute(Attribute::new_matrix44("transform"));
        add_scene_object(world, object);
    }

    pub(super) fn get_all<'a>(world: &'a mut World) -> Vec<SceneObjectEntityRef> {
        world
            .query::<(&SceneObjectEntity, Entity)>()
            .iter(&world)
            .map(|(_, entity)| SceneObjectEntityRef { entity, world })
            .collect::<Vec<_>>()
    }

    pub(super) fn add_scene_object(world: &mut World, object: SceneObject) -> u32 {
        let attributes = AttributeMap::from(object.attributes);
        let links = AttributeLinkMap::new();
        let entity = world.spawn((SceneObjectEntity, object.name, attributes, links));
        entity.id().index()
    }

    pub(super) fn set_scene_object(world: &mut World, id: u32, object: SceneObject) -> Option<u32> {
        let entity = Entity::from_raw(id);
        let Some(mut entity) = world.get_entity_mut(entity) else {
            return None;
        };

        if entity.get::<SceneObjectEntity>().is_none() {
            return None;
        }

        let Some(_) = entity.get::<Name>() else {
            return None;
        };

        let Some(_) = entity.get::<AttributeMap>() else {
            return None;
        };

        let Some(_) = entity.get::<AttributeLinkMap>() else {
            return None;
        };

        entity.insert(object.name);
        entity.insert(AttributeMap::from(object.attributes));

        Some(entity.id().index())
    }

    pub(super) fn remove_scene_object_by_id(world: &mut World, device_id: u32) -> Option<u32> {
        let entity = Entity::from_raw(device_id);

        let Some(_) = world.get_mut::<SceneObjectEntity>(entity) else {
            return None;
        };

        match world.despawn(entity) {
            true => Some(device_id),
            false => None,
        }
    }
}

pub mod commands {

    use super::{system, Command};
    use crate::commands::{CommandError, CommandReply, CommandResult};
    use crate::prelude::Name;
    use crate::world::World;

    pub fn procces(world: &mut World, command: Command) -> CommandResult {
        match command {
            Command::AddObject(object) => {
                let mut query = world.query::<(&system::SceneObjectEntity, &Name)>();
                for (_, name) in query.iter(&world).collect::<Vec<_>>() {
                    if name == &object.name {
                        let reason = format!("object with name '{}' already exists.", object.name);
                        return Err(CommandError::Failed { reason });
                    }
                }
                let id = system::add_scene_object(world, object);
                Ok(Some(CommandReply::EntityId(id)))
            }
            Command::UpdateObject(id, object) => {
                match system::set_scene_object(world, id, object) {
                    Some(id) => Ok(Some(CommandReply::EntityId(id))),
                    None => Err(CommandError::NotFound),
                }
            }
            Command::RemoveObject(id) => match system::remove_scene_object_by_id(world, id) {
                Some(id) => Ok(Some(CommandReply::EntityId(id))),
                None => Err(CommandError::NotFound),
            },
        }
    }
}
