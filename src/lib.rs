pub mod components;
mod error;
pub mod server;

pub static VERSION: &str = "0.1.0";
pub static DEFAULT_GRPC_PORT: u16 = 7638;
pub static DEFAULT_WEB_PORT: u16 = 7272;

pub use error::{Error, Result};
pub use indiemotion_api as api;
pub use tonic::async_trait;
