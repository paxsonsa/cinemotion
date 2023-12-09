use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to complete signaling due to error: {0}")]
    SignalingFailed(String),
}
