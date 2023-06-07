use crate::api;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("client error occurred: {0}")]
    ClientError(String),

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

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::ClientError(_) => tonic::Status::invalid_argument(format!("{}", value)),
            Error::RuntimeError(_) => tonic::Status::internal(format!("{}", value)),
            Error::InvalidRecordingOperation(_) => {
                tonic::Status::failed_precondition(format!("{}", value))
            }
            Error::RuntimeLoopFailed(_) => tonic::Status::failed_precondition(format!("{}", value)),
            Error::InternalError(_) => tonic::Status::internal(format!("{}", value)),
            Error::TonicTransport(_) => value.into(),
            Error::Tonic(_) => value.into(),
            Error::TokioError(_) => value.into(),
        }
    }
}
