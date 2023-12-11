use std::pin::Pin;

use anyhow::Result;
use cinemotion::services::runtime::{RuntimeOptions, RuntimeService};
use cinemotion::webrtc::SignalingRelay;
use clap::Args;
use futures::stream::FuturesUnordered;
use futures::StreamExt;

/// Start the cinemotion broker services.
#[derive(Args)]
pub struct StartCmd {
    #[clap(long = "address")]
    server_bind_address: Option<std::net::SocketAddr>,
}

impl StartCmd {
    pub async fn run(&self) -> Result<i32> {
        tracing::info!("starting...");
        let mut services: Vec<Pin<Box<dyn cinemotion::services::Service>>> = vec![];
        let (cancel_tx, mut cancel_rx) = tokio::sync::mpsc::channel(1);

        let (sender, reciever) = cinemotion::commands::request_pipe();
        let relay = SignalingRelay::new(sender);

        tracing::info!("configure runtime services");
        let runtime = Box::pin(RuntimeService::new(RuntimeOptions {
            request_pipe: reciever,
        }));
        services.push(runtime);

        // Build the default binding addr for the signaling server.
        let bind_addr = self.server_bind_address.unwrap_or(
            format!("0.0.0.0:{}", cinemotion::DEFAULT_WEB_PORT)
                .parse()
                .unwrap(),
        );
        tracing::info!("broadcasting service on: http://{}", bind_addr);

        tracing::debug!("configure the mdns service");
        services.push(Box::pin(cinemotion::services::mdns::MdnsService::new(
            bind_addr.port(),
        )));

        tracing::debug!("configure http service");
        services.push(Box::pin(cinemotion::services::http::HttpService::new(
            bind_addr, relay,
        )));

        let interrupt_task = tokio::task::spawn(async move {
            tracing::debug!("listening for interrupt signals...");
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("interrupt recevied, shutting down...");
                },
            };
            let _ = cancel_tx.send(()).await;
        });

        let mut futures: FuturesUnordered<Pin<Box<dyn cinemotion::services::Service>>> =
            services.drain(..).collect();

        tokio::select! {
            _ = futures.next() => {
                tracing::error!("one or more components exited, shutting down...");
            },
            _ = cancel_rx.recv() => {
                tracing::info!("server shutdown signal received...");
            }
        }

        for service in futures.iter() {
            service.shutdown().await;
        }
        tracing::info!("waiting for the remaining components to shut down...");
        while tokio::time::timeout(tokio::time::Duration::from_secs(3), futures.next())
            .await
            .unwrap_or_default()
            .is_some()
        {}

        interrupt_task.abort();

        Ok(0)
    }
}
