use crate::api;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid operation while recording: {0}")]
    InvalidRecordingOperation(&'static str),

    #[error("Runtime loop failed: {0}")]
    RuntimeLoopFailed(&'static str),

    #[error("Runtime error: {0}")]
    RuntimeError(&'static str),

    #[error("Property update error: property={0} msg={1}")]
    PropertyUpdateError(api::ProperyID, &'static str),

    #[error("internal error: {0}")]
    InternalError(&'static str),

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

impl Into<tonic::Status> for Error {
    fn into(self) -> tonic::Status {
        match self {
            Error::InvalidRecordingOperation(_) => {
                tonic::Status::failed_precondition(format!("{}", self))
            }
            Error::RuntimeLoopFailed(_) => tonic::Status::failed_precondition(format!("{}", self)),
            Error::PropertyUpdateError(_, msg) => tonic::Status::failed_precondition(msg),
            Error::InternalError(_) => tonic::Status::internal(format!("{}", self)),
            Error::APIError(_) => tonic::Status::internal(format!("{}", self)),
            Error::TonicTransport(_) => self.into(),
            Error::Tonic(_) => self.into(),
            Error::TokioError(_) => self.into(),
        }
    }
}
