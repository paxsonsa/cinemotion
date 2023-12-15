use std::{pin::Pin, time::Duration};

use async_trait::async_trait;

use crate::{
    commands::{Request, RequestPipeRx, RequestPipeTx},
    engine::session::SessionComponentImpl,
    engine::Engine,
    Error, Result,
};

use super::Service;

pub struct RuntimeOptions {
    pub request_pipe: (RequestPipeTx, RequestPipeRx),
}

pub struct RuntimeService {
    future: tokio::task::JoinHandle<Result<()>>,
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

impl RuntimeService {
    pub fn new(options: RuntimeOptions) -> Self {
        let mut request_pipe = options.request_pipe.1;
        let session_component = SessionComponentImpl::boxed(options.request_pipe.0.clone());
        let engine = Engine::builder()
            .with_session_component(session_component)
            .build()
            .unwrap();

        let mut engine = Box::new(engine);

        let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);
        let future = tokio::spawn(async move {
            let mut request_buffer: Vec<Request> = Vec::with_capacity(1024);
            let mut interval = tokio::time::interval(Duration::from_millis(16));
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        break;
                    }
                    request = request_pipe.recv() => apply_request(&mut engine, request).await?,
                    _ = interval.tick() => {
                        engine.tick().await?
                    }
                }
            }
            Ok(())
        });
        RuntimeService {
            future,
            shutdown_tx,
        }
    }
}

async fn apply_request(engine: &mut Box<Engine>, request: Option<Request>) -> Result<()> {
    let Some(request) = request else {
        return Err(Error::ChannelClosed("runtime request channel closed."));
    };
    engine.apply(request).await
}

#[async_trait]
impl Service for RuntimeService {
    #[doc = " The name of this component for use in identification and debugging"]
    fn name(&self) -> &'static str {
        "runtime"
    }

    #[doc = " Trigger a shutdown of this component"]
    async fn shutdown(&self) {
        let _ = self.shutdown_tx.send(()).await;
    }
}

impl futures::Future for RuntimeService {
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
