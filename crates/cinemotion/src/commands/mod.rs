mod command;
mod create_session;
mod response;

pub use create_session::CreateSession;

pub use command::Command;
pub use response::Response;

pub type CommandReceiver = tokio::sync::mpsc::UnboundedReceiver<Command>;
pub type CommandSender = tokio::sync::mpsc::UnboundedSender<Command>;

pub type ResponseSender = tokio::sync::broadcast::Sender<Response>;
pub type ResponseReceiver = tokio::sync::broadcast::Receiver<Response>;

pub fn command_channel() -> (CommandSender, CommandReceiver) {
    tokio::sync::mpsc::unbounded_channel()
}

pub fn response_channel() -> ResponseSender {
    let (sender, _) = tokio::sync::broadcast::channel(1024);
    sender
}
