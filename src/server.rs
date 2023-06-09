use futures::stream::FuturesUnordered;
use futures::Future;
use futures::StreamExt;
use std::pin::Pin;

use crate::component;
use crate::Result;

/// Constructor helper for creating a new server
pub struct ServerBuilder {
    /// Public name to advertise for this server.
    name: String,
    engine_builder: Option<component::EngineComponentBuilder>,
    client_service: Option<component::ClientComponentBuilder>,
    web_service: Option<component::WebsocketComponentBuilder>,
}

impl ServerBuilder {
    pub async fn build(&mut self) -> crate::Result<Server> {
        let mut server = Server {
            components: Vec::new(),
        };

        let engine_command_channel = tokio::sync::mpsc::unbounded_channel();
        let state_channel = tokio::sync::mpsc::unbounded_channel();

        let Some(engine_service) = self.engine_builder.take() else {
            panic!("Engine Service not configured")
        };
        let (engine_component, engine_service) = engine_service
            .with_command_rx(engine_command_channel.1)
            .with_state_tx(state_channel.0)
            .build()
            .await?;
        tracing::info!("Engine Service Initialized");
        server.components.push(Box::pin(engine_component));

        let Some(client_service) = self.client_service.take() else {
            todo!();
        };
        let client_service = client_service
            .with_engine_service(engine_service)
            .build()
            .await?;
        let client_proxy = client_service.build_proxy();
        tracing::info!("Client Service Initialized");

        server.components.push(Box::pin(client_service));

        if let Some(web_service) = self.web_service.take() {
            let web_service = web_service.with_client_proxy(client_proxy).build().await?;
            tracing::info!("Web Service Initialized");
            server.components.push(Box::pin(web_service));
        }

        Ok(server)
    }

    pub fn with_client_service(mut self, config: component::ClientComponentBuilder) -> Self {
        self.client_service = Some(config);
        self
    }

    pub fn with_engine_service(mut self, config: component::EngineComponentBuilder) -> Self {
        self.engine_builder = Some(config);
        self
    }

    pub fn with_websocket_service(mut self, config: component::WebsocketComponentBuilder) -> Self {
        self.web_service = Some(config);
        self
    }

    /// Set the name for the server
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

pub struct Server {
    /// Represents a component of the server
    components: Vec<Pin<Box<dyn component::Component>>>,
    // future: tokio::task::JoinHandle<std::result::Result<(), tonic::transport::Error>>,
    // shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl Server {
    /// Create a builder instance to configure and create a new server.
    pub fn builder() -> ServerBuilder {
        ServerBuilder {
            name: "indiemotion".to_string(),
            web_service: None,
            engine_builder: None,
            client_service: None,
        }
    }

    pub async fn serve_with_shutdown(&mut self, shutdown: impl Future<Output = ()>) -> Result<()> {
        let mut components: FuturesUnordered<Pin<Box<dyn component::Component>>> =
            self.components.drain(..).collect();

        tracing::info!("server is running");
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
        while tokio::time::timeout(tokio::time::Duration::from_secs(3), components.next())
            .await
            .unwrap_or_default()
            .is_some()
        {}

        Ok(())
    }
}
