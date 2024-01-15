use quinn::{ClientConfig, Endpoint, ServerConfig};
use std::{error::Error, net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // TODO: Create Simple Server to Echo Messages from Client
    // - It would appear the Swift QUIC in Networking is not capable of customized CA
    // this is an issue. That might mean we need to use Rust Binding with Swift?
    // Another option is to message the Apple Help.
    let addr = "127.0.0.1:4567".parse().unwrap();
    run_server(addr).await;
}

async fn run_server(addr: SocketAddr) {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = cert.serialize_der().unwrap();
    let private_key = cert.serialize_private_key_der();
    let private_key = rustls::PrivateKey(private_key);
    let cert_chain = vec![rustls::Certificate(cert_der.clone())];

    let mut server_config = ServerConfig::with_single_cert(cert_chain, private_key).unwrap();
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());
    transport_config.max_concurrent_bidi_streams(0_u8.into());

    let endpoint = Endpoint::server(server_config, addr).unwrap();

    let incoming = endpoint.accept().await.unwrap();
    let conn = incoming.await.unwrap();
    println!(
        "[server] connection accepted: addr={}",
        conn.remote_address()
    );
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
