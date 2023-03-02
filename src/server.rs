use clap::Args;
use futures::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

use tonic::transport;
use tower_http::trace::TraceLayer;

use crate::proto;
use crate::service::IndieMotionService;

#[derive(Default, Debug, Clone, Args)]
pub struct ServerBuilder {
    /// The local socket address on which to serve the grpc endpoint
    #[clap(long = "server.bind-address")]
    bind_address: Option<std::net::SocketAddr>,
}

impl ServerBuilder {
    pub async fn build(&self) -> crate::Result<Server> {
        let client_manager = Arc::new(Mutex::new(crate::client::ClientManager::default()));
        let runtime = crate::runtime::MotionRuntime::new(client_manager.clone());
        let service = IndieMotionService::new(client_manager, Arc::new(Mutex::new(runtime)));

        let grpc_bind_address = self
            .bind_address
            .clone()
            .unwrap_or_else(|| ([0, 0, 0, 0], 7737).into());

        let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);
        let socket = tokio::net::TcpListener::bind(&grpc_bind_address).await?;
        let grpc_bound = socket.local_addr().ok().unwrap();

        tracing::info!("grpc service listening on {:?}", grpc_bound);
        let future = transport::Server::builder()
            .layer(TraceLayer::new_for_grpc())
            // https://docs.rs/tower-http/latest/tower_http/trace/index.html
            .add_service(proto::indie_motion_service_server::IndieMotionServiceServer::new(service))
            .serve_with_incoming_shutdown(
                tokio_stream::wrappers::TcpListenerStream::new(socket),
                async move {
                    let _ = shutdown_rx.recv().await;
                    tracing::debug!("received shutdown signal for grpc server...");
                },
            );

        Ok(Server {
            future: tokio::task::spawn(future),
            shutdown_tx,
        })
    }
}

pub struct Server {
    future: tokio::task::JoinHandle<std::result::Result<(), tonic::transport::Error>>,
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl Server {
    pub async fn serve_with_shutdown(
        &mut self,
        shutdown: impl Future<Output = ()>,
    ) -> crate::Result<()> {
        tracing::debug!("server is running");
        tokio::select! {
            _ = &mut self.future => {
                tracing::error!("service shutdown...");
            },
            _ = shutdown => {
                tracing::info!("server shutdown signal received...");
                self.shutdown().await?;
            }
        }

        Ok(())
    }

    pub async fn shutdown(&mut self) -> crate::Result<()> {
        let _ = self.shutdown_tx.send(()).await;
        Ok(())
    }
}
