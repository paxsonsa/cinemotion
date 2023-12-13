use crate::Result;

mod command;
mod create_session;
mod event;
mod request;
mod start_session;

pub use create_session::CreateSession;
pub use start_session::StartSession;

pub use command::Command;
pub use event::{Event, EventPayload};
pub use request::Request;

pub type EventPipeTx = tokio::sync::broadcast::Sender<Event>;
pub type EventPipeRx = tokio::sync::broadcast::Receiver<Event>;

pub type RequestPipeTx = tokio::sync::mpsc::UnboundedSender<Request>;
pub type RequestPipeRx = tokio::sync::mpsc::UnboundedReceiver<Request>;

pub fn request_pipe() -> (RequestPipeTx, RequestPipeRx) {
    tokio::sync::mpsc::unbounded_channel()
}

pub fn event_pipe() -> EventPipeTx {
    let (sender, _) = tokio::sync::broadcast::channel(1024);
    sender
}
