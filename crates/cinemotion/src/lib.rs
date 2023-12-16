pub mod commands;
pub mod data;
pub mod engine;
pub(crate) mod error;
pub mod services;
pub mod session;
pub mod webrtc;

pub static VERSION: &str = "0.1.0";
pub static DEFAULT_WEB_PORT: u16 = 7272;

pub use error::{Error, Result};

pub use commands::{Command, Event, Request};
pub use engine::{Engine, EngineState};
pub use session::SessionAgent;
