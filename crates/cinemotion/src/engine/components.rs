use async_trait::async_trait;

use crate::{commands::AddConnection, message, Event, Result};

/// A component that handles all the network operations.
#[async_trait]
pub trait NetworkComponent: Send + Sync {
    /// Get the context for a connection id.
    async fn context_for(&self, conn_id: usize) -> Option<&message::Context>;
    /// Create and add a connection to manage
    async fn add_connection(&mut self, options: AddConnection) -> Result<()>;
    /// Close the connection and stop communicating with the peer.
    async fn close_connection(&mut self, conn_id: usize) -> Result<()>;
    /// Send an event to the connected peers.
    async fn send(&mut self, event: Event) -> Result<()>;
}
