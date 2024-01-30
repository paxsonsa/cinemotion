use super::Service;
use bytes::BytesMut;
use hostname;
use quinn::{Endpoint, ServerConfig};
use std::borrow::BorrowMut;
use std::pin::Pin;
use std::str;
use std::{net::SocketAddr, sync::Arc};

use crate::commands::{AddConnection, MessagePipeTx};
use crate::connection::LOCAL_CONN_ID;
use crate::quic;

pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"cinemotionv1"];

pub struct QuicService {
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
    future: tokio::task::JoinHandle<std::result::Result<(), crate::Error>>,
}

impl QuicService {
    pub fn new(sender: MessagePipeTx) -> Self {
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);
        QuicService {
            shutdown_tx,
            future: tokio::task::spawn(async move {
                tokio::select! {
                    _ = shutdown_rx.recv() => {}
                    _ = run_server(sender) => {}
                }

                Ok(())
            }),
        }
    }

    // Add your methods here
}

#[async_trait::async_trait]
impl Service for QuicService {
    fn name(&self) -> &'static str {
        "quic"
    }

    async fn shutdown(&self) {
        let _ = self.shutdown_tx.send(()).await;
    }
}

impl futures::Future for QuicService {
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

async fn run_server(sender: MessagePipeTx) {
    let addr = "0.0.0.0:4567".parse().unwrap();
    let cert = rcgen::generate_simple_self_signed(get_certificate_names().await).unwrap();
    let cert_der = cert.serialize_der().unwrap();
    let private_key = cert.serialize_private_key_der();
    let private_key = rustls::PrivateKey(private_key);
    let cert_chain = vec![rustls::Certificate(cert_der.clone())];

    let mut server_crypto = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .unwrap();
    server_crypto.alpn_protocols = ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();

    let mut server_config = ServerConfig::with_crypto(Arc::new(server_crypto));

    // Update the idle timeout on the tansport config to ensure the conneciton stays alive.
    // NOTE: this can be a source of lock up issues but those conditions are not yet known.
    let mut transport = quinn::TransportConfig::default();
    transport.max_idle_timeout(None);
    server_config.transport_config(Arc::new(transport));

    // Start the sever endpoint and accept incoming conections.
    let endpoint = Endpoint::server(server_config, addr).unwrap();

    loop {
        // Accept an incoming connection and notify the runtime of the new connection.
        let Some(connecting) = endpoint.accept().await else {
            tracing::error!("failed to accept quic connection");
            continue;
        };

        match connecting.await {
            Ok(conn) => {
                let (ack_pipe, ack_pipe_rx) = tokio::sync::oneshot::channel();
                let agent = Box::new(quic::QuicAgent::new(conn, ack_pipe_rx));
                if let Err(err) = sender.send(crate::Message::with_command(
                    LOCAL_CONN_ID,
                    AddConnection { agent, ack_pipe },
                )) {
                    tracing::error!(%err, "failed to send connection to runtime, closing service.");
                    return;
                }
            }
            Err(err) => {
                tracing::error!(%err, "failed to establish quic connection");
            }
        }
    }
}

async fn get_certificate_names() -> Vec<String> {
    let mut names = vec!["0.0.0.0".into(), "127.0.0.1".into(), "localhost".into()];
    if let Ok(name) = hostname::get() {
        if let Ok(name) = name.into_string() {
            names.push(name);
        }
    }
    // Get all network interfaces
    let ifaces = get_if_addrs::get_if_addrs().expect("Failed to get network interfaces");

    for iface in ifaces {
        // Check if the IP is v4 and the interface is not a loopback
        if !iface.is_loopback() && iface.ip().is_ipv4() {
            names.push(iface.ip().to_string());
        }
    }
    names
}

// Implementation of `ServerCertVerifier` that verifies everything as trustworthy.
struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}
fn configure_client() -> quinn::ClientConfig {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    quinn::ClientConfig::new(Arc::new(crypto))
}
