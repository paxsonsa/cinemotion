use crate::session::SessionAgent;
use crate::Result;

pub struct CreateSession {
    pub agent: Box<dyn SessionAgent + Send + Sync>,
    pub ack_pipe: tokio::sync::oneshot::Sender<Result<()>>,
}
