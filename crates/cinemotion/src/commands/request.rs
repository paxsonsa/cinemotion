use super::Command;

pub struct Request {
    pub session_id: usize,
    pub command: Command,
}

impl Request {
    pub fn with_command(session_id: usize, command: impl Into<Command>) -> Request {
        Request {
            session_id,
            command: command.into(),
        }
    }
}
