use futures::stream::FuturesUnordered;
use futures::Future;
use futures::StreamExt;
use std::pin::Pin;

use crate::async_trait;
use crate::components;
use crate::Result;

/// Represents a component of the server
#[async_trait]
pub trait Component: Future<Output = ()> {
    /// The name of this component for use in identification and debugging
    fn name(&self) -> &'static str;

    /// Trigger a shutdown of this component
    async fn shutdown(&self);
}

/// Constructor helper for creating a new server
pub struct ServerBuilder {
    /// Public name to advertise for this server.
    name: String,

    grpc_service: Option<components::grpc::GrpcServiceBuilder>,
}

impl ServerBuilder {
    pub async fn build(&mut self) -> crate::Result<Server> {
        let mut server = Server {
            components: Vec::new(),
        };

        if let Some(grpc_service) = self.grpc_service.take() {
            let grpc_service = grpc_service.build().await?;
            server.components.push(Box::pin(grpc_service));
        }
        Ok(server)
    }

    /// Change the server name
    pub fn with_grpc_service(mut self, config: components::grpc::GrpcServiceBuilder) -> Self {
        self.grpc_service = Some(config);
        self
    }

    /// Enable the grpc service component with the given configuration
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

pub struct Server {
    /// Represents a component of the server
    components: Vec<Pin<Box<dyn Component>>>,
    // future: tokio::task::JoinHandle<std::result::Result<(), tonic::transport::Error>>,
    // shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl Server {
    /// Create a builder instance to configure and create a new server.
    pub fn builder() -> ServerBuilder {
        ServerBuilder {
            name: "indiemotion".to_string(),
            grpc_service: None,
        }
    }

    pub async fn serve_with_shutdown(&mut self, shutdown: impl Future<Output = ()>) -> Result<()> {
        let mut components: FuturesUnordered<Pin<Box<dyn Component>>> =
            self.components.drain(..).collect();

        tracing::debug!("server is running");
        tokio::select! {
            _ = components.next() => {
                tracing::error!("one or more components exited, shutting down...");
            },
            _ = shutdown => {
                tracing::info!("server shutdown signal received...");
            }
        }

        for component in components.iter() {
            component.shutdown().await;
        }
        tracing::info!("waiting for the remaining components to shut down...");
        while components.next().await.is_some() {}

        Ok(())
    }
}
