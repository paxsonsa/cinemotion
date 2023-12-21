use super::{Command, InternalCommand};
use crate::connection::ConnectionAgent;
use crate::Result;

pub struct OpenConnection {}

impl From<OpenConnection> for Command {
    fn from(value: OpenConnection) -> Self {
        Self::Internal(InternalCommand::OpenConnection(value))
    }
}

pub struct AddConnection {
    pub agent: Box<dyn ConnectionAgent + Send + Sync>,
    pub ack_pipe: tokio::sync::oneshot::Sender<Result<()>>,
}

impl From<AddConnection> for Command {
    fn from(value: AddConnection) -> Self {
        Self::Internal(InternalCommand::AddConnection(value))
    }
}
