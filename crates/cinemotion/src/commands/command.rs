use super::*;
use crate::{Error, Result};

#[derive(Debug)]
pub enum Command {
    System(SystemCommand),
    Controller(ControllerCommand),
}

impl Command {
    /// Decode a command from a byte buffer.
    ///
    /// Note: only client commands can be decoded from a byte buffer.
    pub fn decode(buf: bytes::Bytes) -> Result<Self> {
        let command = ControllerCommand::decode(buf)?;
        Ok(Self::Controller(command))
    }
}

impl From<SystemCommand> for Command {
    fn from(value: SystemCommand) -> Self {
        Self::System(value)
    }
}

impl From<ControllerCommand> for Command {
    fn from(value: ControllerCommand) -> Self {
        Self::Controller(value)
    }
}

/// An internal command is not received from the client but used interally to
/// communicate between service layers with the core engine.
#[derive(Debug)]
pub enum SystemCommand {
    AddConnection(AddConnection),
    OpenConnection(OpenConnection),
}

impl From<AddConnection> for SystemCommand {
    fn from(value: AddConnection) -> Self {
        Self::AddConnection(value)
    }
}

/// Controller commands are received from the controller to
/// control the engine.
#[derive(Debug)]
pub enum ControllerCommand {
    Echo(Echo),
    Init(Init),
    ChangeMode(ChangeMode),
    AddSceneObject(AddSceneObject),
    UpdateSceneObject(UpdateSceneObject),
    DeleteSceneObject(DeleteSceneObject),
    SampleMotion(SampleMotion),
}

impl ControllerCommand {
    /// Decode a command from a byte buffer.
    pub fn decode(buf: bytes::Bytes) -> Result<Self> {
        let Some(payload) = cinemotion_proto::Command::try_from(buf)
            .map_err(|err| Error::BadCommand(format!("failed to decode command: {err}")))?
            .payload
        else {
            return Err(Error::BadCommand("command is missing payload.".to_string()));
        };
        Ok(payload.into())
    }
}

impl From<cinemotion_proto::command::Payload> for ControllerCommand {
    fn from(value: cinemotion_proto::command::Payload) -> Self {
        match value {
            cinemotion_proto::command::Payload::Echo(p) => Self::Echo(p.into()),
            cinemotion_proto::command::Payload::Init(p) => Self::Init(p.into()),
            cinemotion_proto::command::Payload::AddSceneObject(p) => Self::AddSceneObject(p.into()),
            cinemotion_proto::command::Payload::UpdateSceneObject(p) => {
                Self::UpdateSceneObject(p.into())
            }
            cinemotion_proto::command::Payload::DeleteSceneObject(p) => {
                Self::DeleteSceneObject(p.into())
            }
        }
    }
}
