use crate::{api, Result};
use std::fmt::Debug;

pub type CommandResult = tokio::sync::oneshot::Receiver<Result<()>>;

#[derive(Debug)]
pub enum Command {
    Ping(tokio::sync::oneshot::Sender<i64>),
    ConnectAs(api::ClientMetadata),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Ping(_) => write!(f, "Ping()"),
            Command::ConnectAs(client) => write!(f, "ConnectAs({:?})", client),
        }
    }
}

impl Command {
    pub async fn new_connect_as(client: api::ClientMetadata) -> (CommandHandle, CommandResult) {
        CommandHandle::new(Command::ConnectAs(client))
    }
}
#[derive(Debug)]
pub struct CommandHandle {
    pub command: Command,
    pub result: tokio::sync::oneshot::Sender<Result<()>>,
}

impl CommandHandle {
    pub fn new(command: Command) -> (Self, tokio::sync::oneshot::Receiver<Result<()>>) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let handle = Self {
            command,
            result: tx,
        };
        (handle, rx)
    }

    pub fn decompose(self) -> (Command, tokio::sync::oneshot::Sender<Result<()>>) {
        (self.command, self.result)
    }
}
