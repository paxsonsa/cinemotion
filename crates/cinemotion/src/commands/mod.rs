mod command;
mod connection;
mod echo;
mod motion;
mod scene;

pub use command::*;
pub use connection::*;
pub use echo::*;
pub use motion::*;
pub use scene::*;

use super::Event;
use super::Message;
// FIXME: Move this to a more appropriate location
pub type EventPipeTx = tokio::sync::broadcast::Sender<Event>;
pub type EventPipeRx = tokio::sync::broadcast::Receiver<Event>;

// FIXME: Rename this to MessagePipe and move to a more appropriate location
pub type RequestPipeTx = tokio::sync::mpsc::UnboundedSender<Message>;
pub type RequestPipeRx = tokio::sync::mpsc::UnboundedReceiver<Message>;

pub fn request_pipe() -> (RequestPipeTx, RequestPipeRx) {
    tokio::sync::mpsc::unbounded_channel()
}

pub fn event_pipe() -> EventPipeTx {
    let (sender, _) = tokio::sync::broadcast::channel(1024);
    sender
}
