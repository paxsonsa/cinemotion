use indiemotion_api as api;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid operation while recording: {0}")]
    InvalidRecordingOperation(&'static str),

    #[error("Runtime loop failed: {0}")]
    RuntimeLoopFailed(&'static str),

    #[error("Property update error: property={0} msg={1}")]
    PropertyUpdateError(api::ProperyID, &'static str),

    #[error("error from tokio: {0}")]
    TokioError(#[from] tokio::io::Error),

    #[error("error from api: {0}")]
    APIError(#[from] api::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
