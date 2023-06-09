use anyhow::Result;
use clap::Args;
use tokio::signal::unix::{signal, SignalKind};

#[derive(Args, Debug)]
pub struct Server {
    #[clap(long = "name", default_value = "indiemotion")]
    pub server_name: String,

    #[clap(long = "server.bind-address")]
    server_bind_address: Option<std::net::SocketAddr>,
}

impl Server {
    pub async fn run(&self) -> Result<i32> {
        tracing::debug!("Building server...");
        let mut builder = indiemotion::server::Server::builder();

        let engine_builder = indiemotion::component::EngineComponent::builder();
        builder = builder.with_engine_service(engine_builder);

        let relay_builder = indiemotion::component::ClientComponent::builder();
        builder = builder.with_client_service(relay_builder);

        let mut websocket_builder =
            indiemotion::component::websocket::WebsocketComponent::builder();
        websocket_builder = websocket_builder.with_server_bind_address(
            self.server_bind_address
                .unwrap_or_else(|| ([0, 0, 0, 0], indiemotion::DEFAULT_WEB_PORT).into()),
        );
        builder = builder.with_websocket_service(websocket_builder);

        let mut server = builder.build().await?;
        let (shutdown_send, shutdown) = tokio::sync::oneshot::channel();
        let server_future = server.serve_with_shutdown(async move {
            let _ = shutdown.await;
        });
        tracing::debug!("server future is ready");

        let mut sigterm = signal(SignalKind::terminate())?;
        let interrupt_task = tokio::task::spawn(async move {
            tracing::debug!("listening for interrupt signals...");
            tokio::select! {
                _ = sigterm.recv() => {
                    tracing::info!("terminate recevied, shutting down...");
                },
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("interrupt recevied, shutting down...");
                },
            };
            let _ = shutdown_send.send(());
        });

        tracing::info!("ready");
        if let Err(err) = server_future.await {
            tracing::error!(?err, "Server encountered and error");
            interrupt_task.abort();
            Ok(1)
        } else {
            interrupt_task.abort();
            Ok(0)
        }
    }
}
