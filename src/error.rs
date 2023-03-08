use crate::api;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid operation while recording: {0}")]
    InvalidRecordingOperation(&'static str),

    #[error("Runtime loop failed: {0}")]
    RuntimeLoopFailed(&'static str),

    #[error("Invalid Runtime: {0}")]
    InvalidRuntime(&'static str),

    #[error("Property update error: property={0} msg={1}")]
    PropertyUpdateError(api::ProperyID, &'static str),

    #[error("error from api: {0}")]
    APIError(#[from] api::Error),

    // #[error(transparent)]
    // IO(#[from] std::io::Error),
    #[error(transparent)]
    TonicTransport(#[from] tonic::transport::Error),
    #[error(transparent)]
    Tonic(#[from] tonic::Status),
    #[error(transparent)]
    TokioError(#[from] tokio::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
