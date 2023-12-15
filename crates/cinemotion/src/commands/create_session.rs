use crate::session::SessionAgent;
use crate::Result;

use super::Command;

pub struct CreateSession {
    pub agent: Box<dyn SessionAgent + Send + Sync>,
    pub ack_pipe: tokio::sync::oneshot::Sender<Result<()>>,
}

impl From<CreateSession> for Command {
    fn from(value: CreateSession) -> Self {
        Self::CreateSession(value)
    }
}
