mod command;
mod connection;
mod echo;
mod scene;

pub use connection::*;
pub use echo::*;
pub use scene::*;

pub use command::*;

use super::Event;
use super::Message;

pub type EventPipeTx = tokio::sync::broadcast::Sender<Event>;
pub type EventPipeRx = tokio::sync::broadcast::Receiver<Event>;

pub type RequestPipeTx = tokio::sync::mpsc::UnboundedSender<Message>;
pub type RequestPipeRx = tokio::sync::mpsc::UnboundedReceiver<Message>;

pub fn request_pipe() -> (RequestPipeTx, RequestPipeRx) {
    tokio::sync::mpsc::unbounded_channel()
}

pub fn event_pipe() -> EventPipeTx {
    let (sender, _) = tokio::sync::broadcast::channel(1024);
    sender
}
