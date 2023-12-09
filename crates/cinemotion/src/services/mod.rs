use async_trait::async_trait;
use futures::Future;

pub mod http;
pub mod mdns;
pub mod runtime;

/// Represents a component of the server
#[async_trait]
pub trait Service: Future<Output = ()> {
    /// The name of this component for use in identification and debugging
    fn name(&self) -> &'static str;

    /// Trigger a shutdown of this component
    async fn shutdown(&self);
}
