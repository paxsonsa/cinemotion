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
}

pub enum CommandReply {
    EntityId(u32),
}

pub enum Command {
    Device(DeviceCommand),
}

pub enum DeviceCommand {
    Register(DeviceRegister),
}

pub struct DeviceRegister {
    pub name: Name,
    pub attributes: Vec<Attribute>,
}

impl DeviceRegister {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into().into(),
            attributes: Vec::new(),
        }
    }
}

impl Into<Command> for DeviceRegister {
    fn into(self) -> Command {
        Command::Device(self.into())
    }
}

impl Into<DeviceCommand> for DeviceRegister {
    fn into(self) -> DeviceCommand {
        DeviceCommand::Register(self)
    }
}
