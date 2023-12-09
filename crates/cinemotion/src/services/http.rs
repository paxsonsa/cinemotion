use std::{collections::HashMap, convert::Infallible, net::SocketAddr, pin::Pin, sync::Arc};

use async_trait::async_trait;
use tokio::sync::Mutex;
use warp::{self, Filter};

use crate::data::SessionDescriptor;
use crate::webrtc::SignalingRelay;

use super::Service;

pub struct HttpService {
    future: tokio::task::JoinHandle<std::result::Result<(), crate::Error>>,
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl HttpService {
    pub fn new<I>(address: I, connection_manager: SignalingRelay) -> Self
    where
        I: Into<SocketAddr> + Send + 'static,
    {
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);

        let manager = Arc::new(Mutex::new(connection_manager));

        let api = api(manager);
        let routes = api.with(warp::log("cinemotion"));
        let service = warp::serve(routes).run(address);

        HttpService {
            future: tokio::task::spawn(async move {
                tokio::select! {
                    _ = service => {}
                    _ = shutdown_rx.recv() => {}
                }
                Ok(())
            }),
            shutdown_tx,
        }
    }
}

#[async_trait]
impl Service for HttpService {
    fn name(&self) -> &'static str {
        "http"
    }

    async fn shutdown(&self) {
        let _ = self.shutdown_tx.send(()).await;
    }
}

impl futures::Future for HttpService {
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

fn api(
    manager: Arc<Mutex<SignalingRelay>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    root().or(sessions_create(manager))
}

fn root() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end().and_then(handle_root)
}

async fn handle_root() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::with_status(
        warp::reply::html("CineMotion"),
        warp::http::StatusCode::OK,
    ))
}

fn sessions_create(
    manager: Arc<Mutex<SignalingRelay>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("sessions"))
        .and(warp::body::json())
        .and(with_connection_manager(manager))
        .and_then(handle_post_sessions)
}

async fn handle_post_sessions(
    session_desc: SessionDescriptor,
    manager: Arc<Mutex<SignalingRelay>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut manager = manager.lock().await;
    match manager.create(session_desc).await {
        Ok(r) => Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::CREATED,
        )),
        Err(err) => {
            let empty: HashMap<String, String> = HashMap::new();
            Ok(warp::reply::with_status(
                warp::reply::json(&empty),
                warp::http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}

fn with_connection_manager(
    manager: Arc<Mutex<SignalingRelay>>,
) -> impl Filter<Extract = (Arc<Mutex<SignalingRelay>>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || manager.clone())
}
