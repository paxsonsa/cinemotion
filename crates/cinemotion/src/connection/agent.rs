use async_trait::async_trait;

use crate::Event;

use super::SendHandlerFn;

#[async_trait]
pub trait ConnectionAgent: Send + Sync {
    /// Initializes the connection agent and establishes a ready connection.
    async fn initialize(&mut self, send_fn: SendHandlerFn);
    /// Receives an event from the server
    async fn receive(&mut self, event: Event);
    /// Closes the connection agent and its connection to the peer.
    async fn close(&mut self);
}
