pub mod commands;
pub mod connection;
pub mod data;
pub mod engine;
pub(crate) mod error;
pub mod events;
pub mod message;
pub mod services;
pub mod webrtc;

// TODO: Add Peer Attribute Mapping
pub static VERSION: &str = "0.1.0";
pub static DEFAULT_WEB_PORT: u16 = 7272;

pub use error::{Error, Result};

pub use commands::Command;
pub use connection::ConnectionAgent;
pub use engine::{Engine, State};
pub use events::{Event, EventBody};
pub use message::Message;
