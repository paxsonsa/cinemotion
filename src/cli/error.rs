use thiserror::Error;

pub(crate) type CLIResult<T> = std::result::Result<T, crate::Error>;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Must be connected to a server to use this command.")]
    NoConnection,

    #[error("Command Failed: {0}")]
    CommandFailed(String),

    #[error("An error occurred while interacting with the API.")]
    Api(indiemotion_api::Error),
}

impl From<indiemotion_api::Error> for Error {
    fn from(error: indiemotion_api::Error) -> Self {
        Error::Api(error)
    }
}
