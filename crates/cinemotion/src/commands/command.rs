use super::{AddConnection, Echo, Init, OpenConnection};
use crate::{Error, Result};

pub enum Command {
    System(SystemCommand),
    Peer(PeerCommand),
}

impl Command {
    /// Decode a command from a byte buffer.
    ///
    /// Note: only client commands can be decoded from a byte buffer.
    pub fn decode(buf: bytes::Bytes) -> Result<Self> {
        let command = PeerCommand::decode(buf)?;
        Ok(Self::Peer(command))
    }
}

impl From<SystemCommand> for Command {
    fn from(value: SystemCommand) -> Self {
        Self::System(value)
    }
}

impl From<PeerCommand> for Command {
    fn from(value: PeerCommand) -> Self {
        Self::Peer(value)
    }
}

/// An internal command is not received from the client but used interally to
/// communicate between service layers with the core engine.
pub enum SystemCommand {
    AddConnection(AddConnection),
    OpenConnection(OpenConnection),
}

impl From<AddConnection> for SystemCommand {
    fn from(value: AddConnection) -> Self {
        Self::AddConnection(value)
    }
}

/// Client commands are received from the client and are used by the client to
/// control the engine.
pub enum PeerCommand {
    Echo(Echo),
    Init(Init),
}

impl PeerCommand {
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

impl From<cinemotion_proto::command::Payload> for PeerCommand {
    fn from(value: cinemotion_proto::command::Payload) -> Self {
        match value {
            cinemotion_proto::command::Payload::Echo(echo) => Self::Echo(echo.into()),
            cinemotion_proto::command::Payload::Init(init) => Self::Init(init.into()),
        }
    }
}
