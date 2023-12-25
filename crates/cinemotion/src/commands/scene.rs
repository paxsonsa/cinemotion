use super::{Command, ControllerCommand};
use crate::scene;
use cinemotion_proto as proto;

#[derive(Debug)]
pub struct AddSceneObject(pub scene::SceneObject);

impl From<AddSceneObject> for Command {
    fn from(value: AddSceneObject) -> Self {
        Self::Controller(ControllerCommand::AddSceneObject(value))
    }
}

impl From<proto::AddSceneObject> for AddSceneObject {
    fn from(value: proto::AddSceneObject) -> Self {
        Self(value.object.unwrap().into())
    }
}

#[derive(Debug)]
pub struct DeleteSceneObject(pub crate::Name);

impl From<DeleteSceneObject> for Command {
    fn from(value: DeleteSceneObject) -> Self {
        Self::Controller(ControllerCommand::DeleteSceneObject(value))
    }
}

impl From<proto::DeleteSceneObject> for DeleteSceneObject {
    fn from(value: proto::DeleteSceneObject) -> Self {
        Self(value.name.into())
    }
}

#[derive(Debug)]
pub struct UpdateSceneObject(pub scene::SceneObject);

impl From<UpdateSceneObject> for Command {
    fn from(value: UpdateSceneObject) -> Self {
        Self::Controller(ControllerCommand::UpdateSceneObject(value))
    }
}

impl From<proto::UpdateSceneObject> for UpdateSceneObject {
    fn from(value: proto::UpdateSceneObject) -> Self {
        Self(value.object.unwrap().into())
    }
}
