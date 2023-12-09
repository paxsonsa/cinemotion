use super::CreateSession;

pub enum Command {
    Echo(String),
    CreateSession(super::CreateSession),
}

impl From<CreateSession> for Command {
    fn from(value: CreateSession) -> Self {
        Self::CreateSession(value)
    }
}
