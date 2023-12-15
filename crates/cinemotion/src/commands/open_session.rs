use super::{Command, InternalCommand};

pub struct OpenSession {}

impl From<OpenSession> for Command {
    fn from(value: OpenSession) -> Self {
        Self::Internal(InternalCommand::OpenSession(value))
    }
}
