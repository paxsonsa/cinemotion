use crate::Result;

mod command;
mod create_session;
mod echo;
mod event;
mod open_session;
mod request;

pub use create_session::*;
pub use echo::*;
pub use open_session::*;

pub use command::*;
pub use event::*;
pub use request::*;

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
