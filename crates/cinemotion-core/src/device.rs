use std::collections::HashMap;

use bevy_ecs::prelude::Component;

use crate::attributes::Attribute;
use crate::name::Name;

#[cfg(test)]
#[path = "device_test.rs"]
mod device_test;

#[derive(Component, Clone)]
pub struct Device {
    id: Option<u32>,
    name: Name,
    attributes: HashMap<Name, Attribute>,
}

impl Device {
    pub fn new<N: Into<Name>>(name: N) -> Self {
        Self {
            id: None,
            name: name.into(),
            attributes: HashMap::new(),
        }
    }
    pub fn id(&self) -> Option<u32> {
        self.id
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = Some(id);
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
    Update(Device),
}

pub mod commands {
    use super::{Command, Device};
    use crate::commands::{CommandError, CommandReply, CommandResult};
    use crate::prelude::Name;
    use crate::world::{Entity, World};

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

                let id = add_device(world, device);

                Ok(Some(CommandReply::EntityId(id)))
            }
            Command::Update(device) => match set_device(world, device) {
                Some(id) => Ok(Some(CommandReply::EntityId(id))),
                None => Err(CommandError::NotFound),
            },
        }
    }

    pub(super) fn add_device(world: &mut World, device: Device) -> u32 {
        let entity = world.spawn((device.name.clone(), device));
        entity.id().index()
    }

    pub(super) fn set_device(world: &mut World, device: Device) -> Option<u32> {
        let Some(device_id) = device.id else {
            return None;
        };
        let entity = Entity::from_raw(device_id);
        let Some(mut entity) = world.get_entity_mut(entity) else {
            println!("not found 2");
            return None;
        };
        entity.insert(device);
        Some(entity.id().index())
    }
}
