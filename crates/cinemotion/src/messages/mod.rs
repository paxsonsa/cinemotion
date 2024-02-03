mod connection;
mod echo;
mod message;
mod motion;
mod payload;
mod scene;

pub use connection::*;
pub use echo::*;
pub use motion::*;
pub use payload::*;
pub use scene::*;

pub use message::Message;

pub type MessagePipeTx = tokio::sync::mpsc::UnboundedSender<Message>;
pub type MessagePipeRx = tokio::sync::mpsc::UnboundedReceiver<Message>;

pub fn message_pipe() -> (MessagePipeTx, MessagePipeRx) {
    tokio::sync::mpsc::unbounded_channel()
}
