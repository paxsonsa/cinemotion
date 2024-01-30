use crate::commands;
use futures::{task::Context, task::Poll, Future};
use quinn::RecvStream;
use std::pin::Pin;
use thiserror::Error;

pub async fn recv_command(stream: &RecvStream) -> Deserialization {
    Deserialization {}
}

struct Deserialization {}

#[derive(Clone, Debug, Error, PartialEq)]
enum SerializationError {
    #[error("unknown error during serialization: {0}")]
    UnknownError(&'static str),
}

impl Future for Deserialization {
    type Output = Result<commands::Command, SerializationError>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Err(SerializationError::UnknownError("Not Implemented.")))
    }
}
