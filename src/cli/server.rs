use anyhow::Result;
use clap::Args;
use tokio::signal::unix::{signal, SignalKind};

#[derive(Args, Debug)]
pub struct Server {
    #[clap(long = "name", default_value = "indiemotion")]
    pub server_name: String,

    #[clap(flatten)]
    pub web_service: indiemotion::components::websocket::WebsocketServiceBuilder,
}

impl Server {
    pub async fn run(&self) -> Result<i32> {
        tracing::debug!("Building server...");
        let mut builder = indiemotion::server::Server::builder();
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
