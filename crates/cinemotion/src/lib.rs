pub mod commands;
pub mod connection;
pub mod data;
pub mod engine;
pub mod error;
pub mod events;
pub mod message;
pub mod name;
pub mod quic;
pub mod scene;
pub mod services;
pub mod state;
pub mod webrtc;

// TODO: A QUIC Implementation
// - Demo with simple client server between swift and rust
// TODO: Add support for triggers
// TODO: Document the API
// TODO: Update Take Manager to track object object and controller state.
// TODO: Add support for multiple scenes
pub static VERSION: &str = "0.1.0";
pub static DEFAULT_WEB_PORT: u16 = 7272;

pub use error::{Error, Result};

pub use commands::Command;
pub use connection::ConnectionAgent;
pub use engine::Engine;
pub use events::{Event, EventBody};
pub use message::Message;
pub use name::Name;
pub use scene::{Scene, SceneObject};
pub use state::State;
