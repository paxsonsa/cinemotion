use crate::session::Session;
use crate::Result;

pub struct CreateSession {
    pub session: Box<dyn Session + Send>,
    pub ack_pipe: tokio::sync::oneshot::Sender<Result<()>>,
}
