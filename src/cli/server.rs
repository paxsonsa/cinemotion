use anyhow::Result;
use clap::Args;
use tokio::signal::unix::{signal, SignalKind};

#[derive(Args, Debug)]
pub struct Server {
    #[clap(long = "name", default_value = "indiemotion")]
    pub server_name: String,

    #[clap(flatten)]
    pub grpc_service: indiemotion::components::grpc::GrpcServiceBuilder,
}

impl Server {
    pub async fn run(&self) -> Result<i32> {
        configure_logging();
        tracing::debug!("Building server...");
        let mut builder = indiemotion::server::Server::builder();
        builder = builder.with_grpc_service(self.grpc_service.clone());
        builder = builder.with_name(self.server_name.clone());

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

fn configure_logging() {
    use tracing_subscriber::layer::SubscriberExt;
    let config = std::env::var("INDIEMOTION_LOG").unwrap();
    let env_filter = tracing_subscriber::filter::EnvFilter::from(config);
    let registry = tracing_subscriber::Registry::default().with(env_filter);

    let layer = Box::new(
        tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_target(false)
            .without_time(),
    );
    tracing::subscriber::set_global_default(registry.with(layer)).unwrap();
}
