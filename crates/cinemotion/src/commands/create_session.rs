use crate::{
    data::{self, SessionDescriptor},
    Result,
};

pub struct CreateSession {
    pub session_desc: data::SessionDescriptor,
    pub sender: tokio::sync::oneshot::Sender<Result<SessionDescriptor>>,
}
