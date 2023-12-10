use super::Command;

pub struct Request {
    pub command: Command,
}

impl Request {
    pub fn with_command(command: impl Into<Command>) -> Request {
        Request {
            command: command.into(),
        }
    }
}
