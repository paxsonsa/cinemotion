use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("invalid property value: {0}")]
    InvalidValue(String),
}

pub type Result<T> = std::result::Result<T, self::Error>;
