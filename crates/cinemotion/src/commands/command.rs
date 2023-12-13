use super::{CreateSession, StartSession};

pub enum Command {
    Echo(String),
    CreateSession(super::CreateSession),
    StartSession(super::StartSession),
}

impl From<CreateSession> for Command {
    fn from(value: CreateSession) -> Self {
        Self::CreateSession(value)
    }
}

impl From<StartSession> for Command {
    fn from(value: StartSession) -> Self {
        Self::StartSession(value)
    }
}
