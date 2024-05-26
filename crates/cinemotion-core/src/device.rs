use std::collections::HashMap;

use bevy_ecs::prelude::Component;

use crate::attributes::Attribute;
use crate::name::*;
#[cfg(test)]
#[path = "device_test.rs"]
mod device_test;

#[derive(Component, Clone)]
pub struct Device {
    name: Name,
    attributes: HashMap<Name, Attribute>,
}

impl Device {
    pub fn new<N: Into<Name>>(name: N) -> Self {
        Self {
            name: name.into(),
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

pub enum Command {
    Register(Device),
    Update((u32, Device)),
    Remove(u32),
}

pub mod commands {
    use super::{system, Command, Device};
    use crate::commands::{CommandError, CommandReply, CommandResult};
    use crate::prelude::Name;
    use crate::world::World;

    pub fn process(world: &mut World, command: Command) -> CommandResult {
        match command {
            Command::Register(device) => {
                let mut query = world.query::<(&Device, &Name)>();
                for (_, name) in query.iter(&world).collect::<Vec<_>>() {
                    if name == &device.name {
                        let reason = format!("device with name '{}' already exists.", device.name);
                        return Err(CommandError::Failed { reason });
                    }
                }
                let id = system::add_device(world, device);
                Ok(Some(CommandReply::EntityId(id)))
            }
            Command::Update((id, device)) => match system::set_device(world, id, device) {
                Some(id) => Ok(Some(CommandReply::EntityId(id))),
                None => Err(CommandError::NotFound),
            },

            Command::Remove(device_id) => match system::remove_device_by_id(world, device_id) {
                Some(id) => Ok(Some(CommandReply::EntityId(id))),
                None => Err(CommandError::NotFound),
            },
        }
    }
}

pub mod system {
    use super::Device;
    use crate::world::{Entity, World};

    pub(crate) fn get_by_id<'a>(world: &'a mut World, id: u32) -> Option<&Device> {
        let entity = Entity::from_raw(id);
        let Some(entity_ref) = world.get_entity_mut(entity) else {
            return None;
        };
        entity_ref.get::<Device>()
    }

    pub(crate) fn add_device(world: &mut World, device: Device) -> u32 {
        let mut entity = world.spawn(device.name.clone());
        entity.insert(device);
        entity.id().index()
    }

    pub(crate) fn set_device(world: &mut World, device_id: u32, device: Device) -> Option<u32> {
        let entity = Entity::from_raw(device_id);
        let Some(mut entity) = world.get_entity_mut(entity) else {
            return None;
        };
        entity.insert(device);
        Some(entity.id().index())
    }

    pub(crate) fn remove_device_by_id(world: &mut World, device_id: u32) -> Option<u32> {
        let entity = Entity::from_raw(device_id);
        match world.despawn(entity) {
            true => Some(device_id),
            false => None,
        }
    }
}
