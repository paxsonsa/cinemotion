use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    // #[error("An error occurred while interacting with the API.")]
    // Api(indiemotion_api::Error),
}

// impl From<indiemotion_api::Error> for Error {
//     fn from(error: indiemotion_api::Error) -> Self {
//         Error::Api(error)
//     }
// }
