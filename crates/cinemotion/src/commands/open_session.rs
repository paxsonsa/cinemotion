use super::Command;

pub struct OpenSession {}

impl From<OpenSession> for Command {
    fn from(value: OpenSession) -> Self {
        Self::OpenSession(value)
    }
}
