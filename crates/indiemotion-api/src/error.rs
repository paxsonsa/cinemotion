use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "error_type", content = "message")]
pub enum Error {
    #[error("bad message error: {0}")]
    BadMessage(String),

    #[error("command encoder error occurred: {0}")]
    MessageEncoding(String),

    #[error("unexpected error occurred: {0}")]
    UnexpectedError(String),

    #[error("invalid scene object: {0}")]
    InvalidSceneObject(String),

    #[error("invalid property value: {0}")]
    InvalidValue(String),

    #[error("internal server error: {0}")]
    InternalError(String),
}
