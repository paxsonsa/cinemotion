use crate::Result;

mod command;
mod create_session;
mod request;
mod response;

pub use create_session::CreateSession;

pub use command::Command;
pub use request::Request;
pub use response::Response;

pub type ResponsePipeTx = tokio::sync::oneshot::Sender<Result<Option<Response>>>;
pub type ResponsePipeRx = tokio::sync::oneshot::Receiver<Result<Option<Response>>>;

pub type ResponseSender = tokio::sync::broadcast::Sender<Response>;
pub type ResponseReceiver = tokio::sync::broadcast::Receiver<Response>;

pub type RequestPipeTx = tokio::sync::mpsc::UnboundedSender<Request>;
pub type RequestPipeRx = tokio::sync::mpsc::UnboundedReceiver<Request>;

pub fn request_channel() -> (RequestPipeTx, RequestPipeRx) {
    tokio::sync::mpsc::unbounded_channel()
}

pub fn response_channel() -> ResponseSender {
    let (sender, _) = tokio::sync::broadcast::channel(1024);
    sender
}
