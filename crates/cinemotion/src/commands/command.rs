use super::{CreateSession, Echo, OpenSession};
use crate::{Error, Result};

pub enum Command {
    Echo(Echo),
    CreateSession(super::CreateSession),
    OpenSession(super::OpenSession),
}

impl Command {
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

impl From<cinemotion_proto::command::Payload> for Command {
    fn from(value: cinemotion_proto::command::Payload) -> Self {
        match value {
            cinemotion_proto::command::Payload::Echo(echo) => Self::Echo(echo.into()),
        }
    }
}
