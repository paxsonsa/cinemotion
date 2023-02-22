use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid operation while recording: {0}")]
    InvalidRecordingOperation(&'static str),

    #[error("Runtime loop failed: {0}")]
    RuntimeLoopFailed(&'static str),

    #[error("error from tokio: {0}")]
    TokioError(#[from] tokio::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
