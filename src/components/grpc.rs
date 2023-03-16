use clap::Args;
use std::pin::Pin;

use tonic::transport;
use tower_http::trace::TraceLayer;

use crate::async_trait;
use crate::server::Component;
use crate::service;
use crate::Result;
use crate::{engine, runtime};

#[derive(Default, Clone, Args)]
pub struct GrpcServiceBuilder {
    /// The local socket address on which to serve the grpc endpoint
    #[clap(long = "api.bind-address")]
    grpc_bind_address: Option<std::net::SocketAddr>,
}

impl std::fmt::Debug for GrpcServiceBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GrpcServiceBuilder")
            .field("grpc_bind_address", &self.grpc_bind_address)
            .finish()
    }
}

impl GrpcServiceBuilder {
    pub async fn build(mut self) -> Result<GrpcService> {
        let grpc_bind_address = self
            .grpc_bind_address
            .take()
            .unwrap_or_else(|| ([0, 0, 0, 0], crate::DEFAULT_GRPC_PORT).into());

        let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);
        let socket = tokio::net::TcpListener::bind(&grpc_bind_address).await?;
        let grpc_bound = socket.local_addr().ok();

        tracing::info!("establishing runtime");
        let (runtime_handle, runtime_shutdown) = new_runtime().await?;
        let service = service::IndieMotionService::new(runtime_handle);

        tracing::info!("grpc service listening on {:?}", grpc_bound);
        let future = transport::Server::builder()
            .layer(TraceLayer::new_for_grpc())
            // https://docs.rs/tower-http/latest/tower_http/trace/index.html
            .add_service(
                crate::proto::indie_motion_service_server::IndieMotionServiceServer::new(service),
            )
            .serve_with_incoming_shutdown(
                tokio_stream::wrappers::TcpListenerStream::new(socket),
                async move {
                    let _ = shutdown_rx.recv().await;
                    tracing::debug!("received shutdown signal for grpc server...");
                    let _ = runtime_shutdown.send(()).await;
                    tracing::info!("shutdown grpc server...");
                },
            );

        Ok(GrpcService {
            future: tokio::task::spawn(future),
            shutdown_tx,
        })
    }
}

pub struct GrpcService {
    future: tokio::task::JoinHandle<std::result::Result<(), tonic::transport::Error>>,
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl GrpcService {
    pub fn builder() -> GrpcServiceBuilder {
        GrpcServiceBuilder::default()
    }
}

#[async_trait]
impl Component for GrpcService {
    fn name(&self) -> &'static str {
        "api"
    }

    async fn shutdown(&self) {
        // the error case here is that we are already stopped because
        // the receiver end of the channel has closed, which is ok
        let _ = self.shutdown_tx.send(()).await;
    }
}

impl futures::Future for GrpcService {
    type Output = ();

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        use std::task::Poll::*;

        match Pin::new(&mut self.future).poll(cx) {
            Pending => Pending,
            Ready(Ok(Ok(_))) => {
                tracing::info!(name = %self.name(), "component exited");
                Ready(())
            }
            Ready(Ok(Err(err))) => {
                tracing::info!(%err, name = %self.name(), "component failed");
                Ready(())
            }
            Ready(Err(err)) => {
                tracing::error!(%err, name=%self.name(), "component panic'd");
                Ready(())
            }
        }
    }
}

pub async fn new_runtime() -> crate::Result<(runtime::Handle, tokio::sync::mpsc::Sender<()>)> {
    let (shutdown_tx, shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);
    let visitor = Box::<engine::Engine>::default();
    let handle = runtime::Handle::new(visitor, shutdown_rx).await;
    Ok((handle, shutdown_tx))
}
