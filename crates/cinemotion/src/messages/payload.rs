use super::*;
use crate::{Error, Result};

#[derive(Debug)]
pub enum Payload {
    // Error(ErrorMessage),
    System(SystemCommand),
    Client(ClientCommand),
    Invalid,
}

impl Payload {
    pub fn from_protobuf(buf: cinemotion_proto::Command) -> Result<Self> {
        let Some(payload) = buf.payload else {
            return Err(Error::BadCommand(
                "failed to decode command from protobuf, no payload found".to_string(),
            ));
        };
        let command = ClientCommand::from_protobuf(payload)?;
        Ok(Self::Client(command))
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

impl From<SystemCommand> for Payload {
    fn from(value: SystemCommand) -> Self {
        Self::System(value)
    }
}

impl From<ClientCommand> for Payload {
    fn from(value: ClientCommand) -> Self {
        Self::Client(value)
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
pub enum ClientCommand {
    Echo(Echo),
    Init(Init),
    ChangeMode(ChangeMode),
    AddSceneObject(AddSceneObject),
    ClearScene(ClearScene),
    DeleteSceneObject(DeleteSceneObject),
    UpdateSceneObject(UpdateSceneObject),
    SampleMotion(SampleMotion),
}

impl ClientCommand {
    /// Decode a command from a byte buffer.
    pub fn from_protobuf(payload: cinemotion_proto::command::Payload) -> Result<Self> {
        Ok(payload.into())
    }
}

impl From<cinemotion_proto::command::Payload> for ClientCommand {
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
