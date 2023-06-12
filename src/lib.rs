mod clients;
pub mod components;
mod engine;
#[cfg(test)]
#[path = "./engine_integration_test.rs"]
mod engine_integration_test;
pub mod error;
pub mod server;
mod sync;

pub static VERSION: &str = "0.1.0";
pub static DEFAULT_WEB_PORT: u16 = 7272;

pub use error::{Error, Result};
pub use indiemotion_api as api;
