use async_trait::async_trait;

use crate::prelude::*;

/// Scene System for managing the scene state
///
/// The scene system is responsible for keeping the scene data
/// in line with the device system. The core responsibilty is to
/// make sure that an object with attribute links to a device attribute
/// is updated each update cycle.
pub struct SceneSystem {}

#[async_trait]
impl EngineSystem for SceneSystem {
    fn name(&self) -> String {
        "system.scene".to_string()
    }

    async fn update(&mut self, world: &mut bevy_ecs::world::World) -> Result<()> {
        todo!()
    }
}
