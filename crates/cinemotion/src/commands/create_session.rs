use crate::session::SessionAgent;
use crate::Result;

use super::{Command, InternalCommand};

pub struct CreateSession {
    pub agent: Box<dyn SessionAgent + Send + Sync>,
    pub ack_pipe: tokio::sync::oneshot::Sender<Result<()>>,
}

impl From<CreateSession> for Command {
    fn from(value: CreateSession) -> Self {
        Self::Internal(InternalCommand::CreateSession(value))
    }
}
