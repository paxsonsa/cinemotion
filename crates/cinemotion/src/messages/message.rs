use super::Payload;

#[derive(Debug)]
pub struct Message {
    pub source_id: usize,
    pub command: Payload,
}

impl Message {
    pub fn with_command(conn_id: usize, command: impl Into<Payload>) -> Message {
        Self {
            source_id: conn_id,
            command: command.into(),
        }
    }
}
