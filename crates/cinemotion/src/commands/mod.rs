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
pub type EventPipeTx = tokio::sync::broadcast::Sender<Event>;
pub type EventPipeRx = tokio::sync::broadcast::Receiver<Event>;

pub type MessagePipeTx = tokio::sync::mpsc::UnboundedSender<Message>;
pub type MessagePipeRx = tokio::sync::mpsc::UnboundedReceiver<Message>;

pub fn message_pipe() -> (MessagePipeTx, MessagePipeRx) {
    tokio::sync::mpsc::unbounded_channel()
}

pub fn event_pipe() -> EventPipeTx {
    let (sender, _) = tokio::sync::broadcast::channel(1024);
    sender
}
