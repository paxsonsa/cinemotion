use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Attribute type mismatch")]
    AttributeTypeMismatch,

    #[error("Property type mismatch")]
    PropertyTypeMismatch,

    #[error("Property ID is invalid: {0}")]
    InvalidProperyID(String),
}

pub type Result<T> = std::result::Result<T, Error>;