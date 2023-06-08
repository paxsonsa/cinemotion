use crate::async_trait;
use futures::Future;

/// Represents a component of the server
#[async_trait]
pub trait Service: Future<Output = ()> {
    /// The name of this component for use in identification and debugging
    fn name(&self) -> &'static str;

    /// Trigger a shutdown of this component
    async fn shutdown(&self);
}
