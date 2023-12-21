mod command;
mod connection;
mod echo;
mod event;
mod request;

pub use connection::*;
pub use echo::*;

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
