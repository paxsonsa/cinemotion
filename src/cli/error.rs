use thiserror::Error;

pub(crate) type CLIResult<T> = std::result::Result<T, crate::Error>;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Must be connected to a server to use this command.")]
    NoConnection,

    #[error("Command Failed: {0}")]
    CommandFailed(String),

    #[error("An error occurred while interacting with the API.")]
    ApiError(indiemotion_api::Error),

    #[error("{0}")]
    ReplError(indiemotion_repl::Error),
}

impl From<indiemotion_repl::Error> for Error {
    fn from(error: indiemotion_repl::Error) -> Self {
        Error::ReplError(error)
    }
}

impl From<indiemotion_api::Error> for Error {
    fn from(error: indiemotion_api::Error) -> Self {
        Error::ApiError(error)
    }
}
