use std::ops::Deref;

use bevy_ecs::prelude::Component;

use crate::attributes::Attribute;
use crate::name::*;
use crate::prelude::AttributeMap;
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
    pub name: Name,
    pub attributes: AttributeMap,
}

impl Device {
    pub fn new<N: Into<Name>>(name: N) -> Self {
        Self {
            name: name.into(),
            attributes: AttributeMap::new(),
        }
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
    use crate::{
        globals,
        world::{Entity, World},
    };

    #[derive(Component)]
    struct DeviceEntity;

    pub struct DeviceEntityRef {
        pub entity: Entity,
    }

    impl DeviceEntityRef {
        pub fn name<'w>(&self, world: &'w World) -> &'w Name {
            world
                .get::<Name>(self.entity)
                .expect("expect device entity to have a name")
        }

        pub fn set_name(&mut self, world: &mut World, name: Name) {
            self._set(world, name)
        }

        pub fn attribute<'w>(&self, world: &'w World, name: &Name) -> Option<&'w Attribute> {
            self.attributes(&world).get(name)
        }

        pub fn set_attributes(&mut self, world: &mut World, attrs: AttributeMap) {
            self._set(world, attrs)
        }
        pub fn attributes<'w>(&self, world: &'w World) -> &'w AttributeMap {
            world
                .get::<AttributeMap>(self.entity)
                .expect("device entity should have attribute map")
        }

        pub fn insert_attribute(&mut self, world: &mut World, attribute: Attribute) {
            world
                .get_mut::<AttributeMap>(self.entity)
                .expect("device entity should have attribute map")
                .insert(attribute)
        }

        fn _set<'w, T: Component>(&mut self, world: &'w mut World, value: T) {
            world.get_entity_mut(self.entity).unwrap().insert(value);
        }
    }

    pub(crate) fn get_by_id<'a>(world: &'a mut World, id: &DeviceId) -> Option<DeviceEntityRef> {
        let entity = Entity::from_raw(**id);
        let Some(entity_ref) = world.get_entity_mut(entity) else {
            return None;
        };
        if entity_ref.get::<DeviceEntity>().is_none() {
            return None;
        }

        Some(DeviceEntityRef { entity })
    }

    pub(super) fn get_all<'a>(world: &'a mut World) -> Vec<DeviceEntityRef> {
        world
            .query::<(&DeviceEntity, Entity)>()
            .iter(&world)
            .map(|(_, entity)| DeviceEntityRef { entity })
            .collect::<Vec<_>>()
    }

    pub(crate) fn add_device(world: &mut World, device: Device) -> DeviceId {
        let entity = world.spawn((DeviceEntity, device.name, device.attributes));
        entity.id().index().into()
    }

    pub(crate) fn set_device(
        world: &mut World,
        device_id: DeviceId,
        device: Device,
    ) -> Option<DeviceId> {
        let motion_enabled = globals::system::is_motion_enabled(world);
        println!("motion_enable: {:?}", motion_enabled);
        let Some(mut device_ref) = get_by_id(world, &device_id) else {
            return None;
        };

        device_ref.set_name(world, device.name);
        if motion_enabled {
            device_ref.set_attributes(world, device.attributes);
        }

        Some(device_id)
    }

    pub(crate) fn remove_device_by_id(world: &mut World, device_id: DeviceId) -> Option<DeviceId> {
        let entity = Entity::from_raw(*device_id);
        match world.despawn(entity) {
            true => Some(device_id),
            false => None,
        }
    }
}
