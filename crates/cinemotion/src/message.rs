use super::Command;

#[derive(Debug)]
pub struct Context {}

#[derive(Debug)]
pub struct Message {
    pub source_id: usize,
    pub command: Command,
}

impl Message {
    pub fn with_command(conn_id: usize, command: impl Into<Command>) -> Message {
        Message {
            source_id: conn_id,
            command: command.into(),
        }
    }
}
