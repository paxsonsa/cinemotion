use super::Command;

pub struct Request {
    pub conn_id: usize,
    pub command: Command,
}

impl Request {
    pub fn with_command(conn_id: usize, command: impl Into<Command>) -> Request {
        Request {
            conn_id,
            command: command.into(),
        }
    }
}
