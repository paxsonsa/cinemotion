use bevy_ecs::prelude::*;
use bevy_ecs::system::{ReadOnlySystemParam, SystemParam, SystemParamItem, SystemState};

use crate::commands::{self, CommandError, CommandInfo, CommandReply, CommandResult};
use crate::error::*;
use crate::prelude::*;
use crate::state::*;

#[cfg(test)]
#[path = "engine_test.rs"]
mod engine_test;

macro_rules! invoke {
    ($option:expr, $method:ident $(, $args:expr)*) => {
        if let Some(ref value) = $option {
            value.$method($($args),*).await?;
        }
    };
}

pub struct EngineState<'a, Param: SystemParam + 'static> {
    system_state: SystemState<Param>,
    param_item: SystemParamItem<'a, 'a, Param>,
}

struct Engine {
    world: World,
    scene_controller: Option<Box<dyn EngineSystem>>,
}

impl Engine {
    fn new() -> Self {
        Engine {
            world: World::new(),
            scene_controller: None,
        }
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn setup(&mut self) -> Result<()> {
        self.world = World::new();
        Ok(())
    }

    async fn tick(&mut self) -> Result<()> {
        // self.device_controller.update(&mut self.world).await?;
        // invoke!(self.scene_controller, update, &mut self.world);
        // self.take_controller.update(&mut self.world).await?;
        // self.render_controller.update(&mut self.world).await?;
        // self.input_controller.update(&mut self.world).await?;
        Ok(())
    }

    async fn process(&mut self, dispatch: CommandInfo) -> Result<()> {
        let CommandInfo { command, dispatch } = dispatch;
        let result = match command {
            commands::Command::Device(c) => handle_device_command(&mut self.world, c),
            // commands::Command::Engine(c) => self.root_system(c)?,
            // commands::Command::Scene(c) => self.scene_system.process(c)?,
            // commands::Command::Object(c) => self.object_system.process(c)?,
            // commands::Command::Take(c) => self.take_system.process(c)?,
        };
        dispatch(result);

        Ok(())
    }

    async fn serialize(&mut self) -> StateTree {
        let state = StateTree::new();
        //
        // for device in self.world.query::<(&Device)>().iter() {
        //     state.devices.push(device)
        // }

        state
    }
}

fn handle_device_command(world: &mut World, command: commands::DeviceCommand) -> CommandResult {
    match command {
        commands::DeviceCommand::Register(mut data) => {
            let mut query = world.query::<(&Device, &Name)>();
            for (_, name) in query.iter(&world).collect::<Vec<_>>() {
                if name == &data.name {
                    let reason = format!("device with name '{}' already exists.", data.name);
                    return Err(CommandError::Failed { reason });
                }
            }

            let mut device = Device::new(data.name.clone());
            for attr in data.attributes.drain(..) {
                let _ = device.insert_attribute(attr);
            }

            let entity = world.spawn((data.name.clone(), device));
            let id = entity.id().index();
            Ok(Some(CommandReply::EntityId(id)))
        }
    }
}
