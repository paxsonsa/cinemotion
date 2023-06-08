
use thiserror::Error;

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
    #[error("internal error: {0}")]
    InternalError(&'static str),

    // #[error("error from api: {0}")]
    // APIError(#[from] api::Error),

    // #[error(transparent)]
    // IO(#[from] std::io::Error),
    #[error(transparent)]
    TonicTransport(#[from] tonic::transport::Error),
    #[error(transparent)]
    Tonic(#[from] tonic::Status),
    #[error(transparent)]
    TokioError(#[from] tokio::io::Error),
}
