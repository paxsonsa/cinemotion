use super::*;
use crate::{quic, Error, Result};

#[derive(Debug)]
pub enum Command {
    System(SystemCommand),
    Controller(ControllerCommand),
    Invalid,
}

impl Command {
    pub fn from_protobuf(buf: cinemotion_proto::Command) -> Result<Self> {
        let Some(payload) = buf.payload else {
            return Err(Error::BadCommand(
                "failed to decode command from protobuf, no payload found".to_string(),
            ));
        };
        let command = ControllerCommand::from_protobuf(payload)?;
        Ok(Self::Controller(command))
    }

    /// Decode a command from a byte buffer.
    ///
    /// Note: only client commands can be decoded from a byte buffer.
    pub fn from_protobuf_bytes(buf: bytes::Bytes) -> Result<Self> {
        let command = cinemotion_proto::Command::try_from(buf)
            .map_err(|err| Error::BadCommand(format!("failed to decode command: {err}")))?;
        Self::from_protobuf(command)
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
    CloseConnection(CloseConnection),
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
    ClearScene(ClearScene),
    DeleteSceneObject(DeleteSceneObject),
    UpdateSceneObject(UpdateSceneObject),
    SampleMotion(SampleMotion),
}

impl ControllerCommand {
    /// Decode a command from a byte buffer.
    pub fn from_protobuf(payload: cinemotion_proto::command::Payload) -> Result<Self> {
        Ok(payload.into())
    }
}

impl From<cinemotion_proto::command::Payload> for ControllerCommand {
    fn from(value: cinemotion_proto::command::Payload) -> Self {
        match value {
            cinemotion_proto::command::Payload::Echo(p) => Self::Echo(p.into()),
            cinemotion_proto::command::Payload::Init(p) => Self::Init(p.into()),
            cinemotion_proto::command::Payload::AddSceneObject(p) => Self::AddSceneObject(p.into()),
            cinemotion_proto::command::Payload::ClearScene(p) => Self::ClearScene(p.into()),
            cinemotion_proto::command::Payload::DeleteSceneObject(p) => {
                Self::DeleteSceneObject(p.into())
            }
            cinemotion_proto::command::Payload::UpdateSceneObject(p) => {
                Self::UpdateSceneObject(p.into())
            }
            cinemotion_proto::command::Payload::ChangeMode(mode) => Self::ChangeMode(mode.into()),
            cinemotion_proto::command::Payload::SendSample(sample) => {
                Self::SampleMotion(sample.into())
            }
        }
    }
}
