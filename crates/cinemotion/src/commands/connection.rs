use super::{ClientCommand, Command, EventPayload, InternalCommand};
use crate::connection::ConnectionAgent;
use crate::data::peer;
use crate::Result;
use cinemotion_proto as proto;

pub struct Init {
    pub peer: peer::Peer,
}

impl From<Init> for Command {
    fn from(value: Init) -> Self {
        Self::Client(ClientCommand::Init(value))
    }
}

impl From<proto::InitCommand> for Init {
    fn from(value: proto::InitCommand) -> Self {
        Self {
            peer: value.peer.unwrap().into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionOpened {}

impl From<ConnectionOpened> for EventPayload {
    fn from(value: ConnectionOpened) -> Self {
        Self::ConnectionOpened(value)
    }
}

impl From<ConnectionOpened> for proto::ConnectionOpenedEvent {
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
