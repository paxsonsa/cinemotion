use super::{ClientCommand, Payload};
use crate::scene;
use cinemotion_proto as proto;

#[derive(Debug)]
pub struct AddSceneObject(pub scene::SceneObject);

impl From<AddSceneObject> for Payload {
    fn from(value: AddSceneObject) -> Self {
        Self::Client(ClientCommand::AddSceneObject(value))
    }
}

impl From<proto::AddSceneObject> for AddSceneObject {
    fn from(value: proto::AddSceneObject) -> Self {
        Self(value.object.unwrap().into())
    }
}

#[derive(Debug)]
pub struct ClearScene {}

impl From<proto::ClearScene> for ClearScene {
    fn from(_: proto::ClearScene) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct DeleteSceneObject(pub crate::Name);

impl From<DeleteSceneObject> for Payload {
    fn from(value: DeleteSceneObject) -> Self {
        Self::Client(ClientCommand::DeleteSceneObject(value))
    }
}

impl From<proto::DeleteSceneObject> for DeleteSceneObject {
    fn from(value: proto::DeleteSceneObject) -> Self {
        Self(value.name.into())
    }
}

#[derive(Debug)]
pub struct UpdateSceneObject(pub scene::SceneObject);

impl From<UpdateSceneObject> for Payload {
    fn from(value: UpdateSceneObject) -> Self {
        Self::Client(ClientCommand::UpdateSceneObject(value))
    }
}

impl From<proto::UpdateSceneObject> for UpdateSceneObject {
    fn from(value: proto::UpdateSceneObject) -> Self {
        Self(value.object.unwrap().into())
    }
}
