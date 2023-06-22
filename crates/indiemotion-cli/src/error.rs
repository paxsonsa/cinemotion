use thiserror::Error;

use crate::api;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("client error occurred: {0}")]
    ClientError(String),

    #[error("client not found: {0}")]
    ClientNotFound(u32),

    #[error("Invalid operation while recording: {0}")]
    InvalidRecordingOperation(&'static str),

    #[error("Runtime loop failed: {0}")]
    RuntimeLoopFailed(&'static str),

    #[error("Runtime error: {0}")]
    RuntimeError(&'static str),

    // #[error("Property update error: property={0} msg={1}")]
    // PropertyUpdateError(api::ProperyID, &'static str),
    #[error("transport error: {0}")]
    TransportError(&'static str),

    #[error("api error: {0}")]
    APIError(#[from] api::Error),

    // #[error(transparent)]
    // IO(#[from] std::io::Error),
    #[error(transparent)]
    TokioError(#[from] tokio::io::Error),
}