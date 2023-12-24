use super::{Command, ControllerCommand};
use crate::scene;

pub struct UpdateSceneObject {
    pub object: scene::SceneObject,
}

impl From<UpdateSceneObject> for Command {
    fn from(value: UpdateSceneObject) -> Self {
        Self::Controller(ControllerCommand::UpdateSceneObject(value))
    }
}
