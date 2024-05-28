use std::collections::HashMap;
use std::ops::Deref;

use bevy_ecs::prelude::Component;

use crate::attributes::Attribute;
use crate::name::*;
#[cfg(test)]
#[path = "device_test.rs"]
mod device_test;

#[derive(Debug, Clone)]
pub struct DeviceId(u32);

impl DeviceId {
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl Deref for DeviceId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u32> for DeviceId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

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

    pub fn attribute(&self, name: &Name) -> Option<&Attribute> {
        self.attributes().get(name)
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
    Update((DeviceId, Device)),
    Remove(DeviceId),
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
                Ok(Some(CommandReply::EntityId(*id)))
            }
            Command::Update((id, device)) => match system::set_device(world, id, device) {
                Some(id) => Ok(Some(CommandReply::EntityId(*id))),
                None => Err(CommandError::NotFound),
            },

            Command::Remove(device_id) => match system::remove_device_by_id(world, device_id) {
                Some(id) => Ok(Some(CommandReply::EntityId(*id))),
                None => Err(CommandError::NotFound),
            },
        }
    }
}

pub mod system {

    use super::*;
    use crate::world::{Entity, World};

    pub(crate) fn get_by_id<'a>(world: &'a mut World, id: &DeviceId) -> Option<Device> {
        let entity = Entity::from_raw(**id);
        let Some(entity_ref) = world.get_entity_mut(entity) else {
            return None;
        };
        entity_ref.get::<Device>().cloned()
    }

    pub(crate) fn add_device(world: &mut World, device: Device) -> DeviceId {
        let mut entity = world.spawn(device.name.clone());
        entity.insert(device);
        entity.id().index().into()
    }

    pub(crate) fn set_device(
        world: &mut World,
        device_id: DeviceId,
        device: Device,
    ) -> Option<DeviceId> {
        let entity = Entity::from_raw(*device_id);
        let Some(mut entity) = world.get_entity_mut(entity) else {
            return None;
        };
        entity.insert(device);
        Some(entity.id().index().into())
    }

    pub(crate) fn remove_device_by_id(world: &mut World, device_id: DeviceId) -> Option<DeviceId> {
        let entity = Entity::from_raw(*device_id);
        match world.despawn(entity) {
            true => Some(device_id),
            false => None,
        }
    }
}
