use super::{Command, PeerCommand, SystemCommand};
use crate::connection::ConnectionAgent;
use crate::data::peer;
use crate::Result;
use cinemotion_proto as proto;

pub struct Init {
    pub peer: peer::Peer,
}

impl From<Init> for Command {
    fn from(value: Init) -> Self {
        Self::Peer(PeerCommand::Init(value))
    }
}

impl From<proto::InitCommand> for Init {
    fn from(value: proto::InitCommand) -> Self {
        Self {
            peer: value.peer.unwrap().into(),
        }
    }
}
pub struct OpenConnection {}

impl From<OpenConnection> for Command {
    fn from(value: OpenConnection) -> Self {
        Self::System(SystemCommand::OpenConnection(value))
    }
}

pub struct AddConnection {
    pub agent: Box<dyn ConnectionAgent + Send + Sync>,
    pub ack_pipe: tokio::sync::oneshot::Sender<Result<usize>>,
}

impl From<AddConnection> for Command {
    fn from(value: AddConnection) -> Self {
        Self::System(SystemCommand::AddConnection(value))
    }
}
