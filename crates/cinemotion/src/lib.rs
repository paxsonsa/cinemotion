pub mod commands;
pub mod connection;
pub mod data;
pub mod engine;
pub mod error;
pub mod events;
pub mod message;
pub mod name;
pub mod scene;
pub mod quic;
pub mod services;
pub mod state;
pub mod webrtc;

// TODO: Add support for animation capture and saving
// 1) Add a new type for storing object and controller animation as takes.
// 2) Create a component for measuring animation takes and tools for creating takes.
// TODO: Add support for triggers
// TODO: Document the API
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
