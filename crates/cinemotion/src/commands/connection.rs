use super::{ClientCommand, Command, EventPayload, InternalCommand};
use crate::connection::ConnectionAgent;
use crate::Result;
use cinemotion_proto::ConnectionOpened as ConnectionOpenedProto;
use cinemotion_proto::Init as InitProto;

pub struct ConnectionInit {
    pub name: String,
}

impl From<ConnectionInit> for Command {
    fn from(value: ConnectionInit) -> Self {
        Self::Client(ClientCommand::Init(value))
    }
}

impl From<InitProto> for ConnectionInit {
    fn from(value: InitProto) -> Self {
        Self { name: value.name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionOpened {}

impl From<ConnectionOpened> for EventPayload {
    fn from(value: ConnectionOpened) -> Self {
        Self::ConnectionOpened(value)
    }
}

impl From<ConnectionOpened> for ConnectionOpenedProto {
    fn from(_: ConnectionOpened) -> Self {
        Self {}
    }
}

pub struct OpenConnection {}

impl From<OpenConnection> for Command {
    fn from(value: OpenConnection) -> Self {
        Self::Internal(InternalCommand::OpenConnection(value))
    }
}

pub struct AddConnection {
    pub agent: Box<dyn ConnectionAgent + Send + Sync>,
    pub ack_pipe: tokio::sync::oneshot::Sender<Result<usize>>,
}

impl From<AddConnection> for Command {
    fn from(value: AddConnection) -> Self {
        Self::Internal(InternalCommand::AddConnection(value))
    }
}
