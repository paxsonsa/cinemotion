use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Attribute type mismatch")]
    AttributeTypeMismatch,

    #[error("Property type mismatch")]
    PropertyTypeMismatch
}

pub type Result<T> = std::result::Result<T, Error>;