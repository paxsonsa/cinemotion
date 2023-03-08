use crate::{api, Result};
use std::fmt::{Debug, Display};

pub type CommandResult = tokio::sync::oneshot::Receiver<Result<()>>;

pub trait CommandType: Display + Clone + Debug + Send + 'static {}

#[async_trait::async_trait]
pub trait CommandBuilder {
    type Command: CommandType;
    async fn new_ping() -> (CommandHandle<Self::Command>, CommandResult);

    async fn new_connect_as(
        client: api::ClientMetadata,
    ) -> (CommandHandle<Self::Command>, CommandResult);
}

#[derive(Debug)]
pub struct CommandHandle<CType>
where
    CType: CommandType,
{
    pub command: CType,
    pub result: tokio::sync::oneshot::Sender<Result<()>>,
}

impl<CType> CommandHandle<CType>
where
    CType: CommandType,
{
    pub fn new(command: CType) -> (Self, tokio::sync::oneshot::Receiver<Result<()>>) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let handle = Self {
            command,
            result: tx,
        };
        (handle, rx)
    }

    pub fn decompose(self) -> (CType, tokio::sync::oneshot::Sender<Result<()>>) {
        (self.command, self.result)
    }
}
