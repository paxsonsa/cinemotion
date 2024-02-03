use async_trait::async_trait;

use crate::{connection, messages::AddConnection, Event, Result};

/// A component that handles all the network operations.
#[async_trait]
pub trait NetworkComponent: Send + Sync {
    // Get a mutable reference to the context for a connection id.
    fn context_mut(&mut self, conn_id: usize) -> &mut connection::Context;
    /// Get the context for a connection id.
    fn context(&self, conn_id: usize) -> Option<&connection::Context>;
    /// Create and add a connection to manage
    async fn add_connection(&mut self, options: AddConnection) -> Result<()>;
    /// Close the connection and stop communicating with the peer.
    async fn close_connection(&mut self, conn_id: usize) -> Result<()>;
    /// Send an event to the connected peers.
    async fn send(&mut self, event: Event) -> Result<()>;
}
