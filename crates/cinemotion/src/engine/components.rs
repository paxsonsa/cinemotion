use async_trait::async_trait;

use crate::{
    commands::{AddConnection, Event},
    Result,
};

/// A component that handles all the network operations.
#[async_trait]
pub trait NetworkComponent: Send + Sync {
    /// Create and add a connection to manage
    async fn add_connection(&mut self, options: AddConnection) -> Result<()>;
    /// Open the connection session and being communicating with the peer.
    async fn open_connection(&mut self, session_id: usize) -> Result<()>;
    /// Close the connection session and stop communicating with the peer.
    async fn close_connection(&mut self, session_id: usize) -> Result<()>;
    /// Send an event to the connected peers.
    async fn send(&mut self, event: Event) -> Result<()>;
}
