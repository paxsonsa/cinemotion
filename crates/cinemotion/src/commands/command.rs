use super::{AddConnection, ConnectionInit, Echo, OpenConnection};
use crate::{Error, Result};

pub enum Command {
    Internal(InternalCommand),
    Client(ClientCommand),
}

impl Command {
    /// Decode a command from a byte buffer.
    ///
    /// Note: only client commands can be decoded from a byte buffer.
    pub fn decode(buf: bytes::Bytes) -> Result<Self> {
        let command = ClientCommand::decode(buf)?;
        Ok(Self::Client(command))
    }
}

impl From<InternalCommand> for Command {
    fn from(value: InternalCommand) -> Self {
        Self::Internal(value)
    }
}

impl From<ClientCommand> for Command {
    fn from(value: ClientCommand) -> Self {
        Self::Client(value)
    }
}

/// An internal command is not received from the client but used interally to
/// communicate between service layers with the core engine.
pub enum InternalCommand {
    AddConnection(AddConnection),
    OpenConnection(OpenConnection),
}

impl From<AddConnection> for InternalCommand {
    fn from(value: AddConnection) -> Self {
        Self::AddConnection(value)
    }
}

/// Client commands are received from the client and are used by the client to
/// control the engine.
pub enum ClientCommand {
    Echo(Echo),
    Init(ConnectionInit),
}

impl ClientCommand {
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

impl From<cinemotion_proto::command::Payload> for ClientCommand {
    fn from(value: cinemotion_proto::command::Payload) -> Self {
        match value {
            cinemotion_proto::command::Payload::Echo(echo) => Self::Echo(echo.into()),
            cinemotion_proto::command::Payload::Init(init) => Self::Init(init.into()),
        }
    }
}
