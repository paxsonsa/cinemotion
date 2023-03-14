pub mod components;
mod engine;
mod error;
mod runtime;
pub mod server;
mod service;

pub static VERSION: &str = "0.1.0";
pub static DEFAULT_GRPC_PORT: u16 = 7638;

pub use error::{Error, Result};
pub use indiemotion_api as api;
pub use indiemotion_proto as proto;
pub use tonic::async_trait;
