use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("command encoder error occurred: {0}")]
    CommandEncoderError(String),
}
