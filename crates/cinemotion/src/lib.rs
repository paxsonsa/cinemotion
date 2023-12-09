pub mod commands;
pub mod data;
pub(crate) mod error;
pub mod services;
pub mod webrtc;

pub static VERSION: &str = "0.1.0";
pub static DEFAULT_WEB_PORT: u16 = 7272;

pub use error::{Error, Result};
