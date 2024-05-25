use std::collections::HashMap;

use thiserror::Error;
use tokio::sync::oneshot;

use crate::prelude::*;

pub type CommandResult = std::result::Result<Option<CommandReply>, CommandError>;
pub type CommandDispatch = Box<dyn FnOnce(CommandResult)>;

pub struct CommandInfo {
    pub command: Command,
    pub dispatch: CommandDispatch,
}

impl CommandInfo {
    pub fn with(command: impl Into<Command>) -> (Self, oneshot::Receiver<CommandResult>) {
        let (sender, receiver) = oneshot::channel();
        let dispatch = Box::new(move |r| {
            let _ = sender.send(r);
        });
        (
            Self {
                command: command.into(),
                dispatch,
            },
            receiver,
        )
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum CommandError {
    #[error("command failed: {reason}")]
    Failed { reason: String },

    #[error("entity does not exist")]
    NotFound,
}

pub enum CommandReply {
    EntityId(u32),
}

pub enum Command {
    Device(device::Command),
}

impl From<device::Command> for Command {
    fn from(value: device::Command) -> Self {
        Self::Device(value)
    }
}
