use super::stream;
use crate::{
    connection::{ConnectionAgent, SendHandlerFn},
    Event, Result,
};
use arc_swap::ArcSwapOption;
use futures::lock::Mutex;
use std::sync::Arc;
use tokio::sync::oneshot::Receiver;

pub struct QuicAgent {
    conn: quinn::Connection,
    send_handler: Arc<ArcSwapOption<Mutex<SendHandlerFn>>>,
}

impl QuicAgent {
    pub fn new(conn: quinn::Connection, ack_pipe: Receiver<Result<usize>>) -> Self {
        let shared_conn = conn.clone();

        // Create a task to handle the ack pipe and close the
        // connection if the ack fails.
        tokio::spawn(async move {
            match ack_pipe.await {
                Ok(result) => match result {
                    Ok(id) => {
                        tracing::info!("quic connection agent added, id={id}");
                    }
                    Err(err) => {
                        tracing::error!("engine failed to add connection: {}", err);
                        shared_conn
                            .close(1u32.into(), "engine failed to ack connection.".as_bytes());
                    }
                },
                Err(e) => {
                    tracing::error!("quic connection agent failed to add: {}", e);
                    shared_conn.close(1u32.into(), "engine failed to ack connection.".as_bytes());
                }
            }
        });
        Self {
            conn,
            send_handler: Arc::new(ArcSwapOption::default()),
        }
    }
}

#[async_trait::async_trait]
impl ConnectionAgent for QuicAgent {
    #[doc = r" Initializes the connection agent and establishes a ready connection."]
    async fn initialize(&mut self, send_fn: SendHandlerFn) {
        self.send_handler.store(Some(Arc::new(Mutex::new(send_fn))));

        // Open a new bidirectional data stream for the message pipe.
        let (send_stream, mut recv_stream) = match self.conn.open_bi().await {
            Ok(stream) => stream,
            Err(e) => {
                tracing::error!("failed to open stream: {}", e);
                return;
            }
        };

        // Start recv loop for the message pipe.
        let shared_send_fn = Arc::clone(&self.send_handler);
        let (shutdown_tx, mut shutdown_rx): (
            tokio::sync::mpsc::Sender<()>,
            tokio::sync::mpsc::Receiver<()>,
        ) = tokio::sync::mpsc::channel(1);
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    serialization = stream::recv_command(&mut recv_stream) => {
                        match serialization {
                            Ok(command) => {
                                if let Some(handler) = &*shared_send_fn.load() {
                                    let mut f = handler.lock().await;
                                    let _ = f(command);
                                }
                            },
                            Err(err) => {
                                tracing::error!("failed to read message: {}", err);
                                continue;
                            }
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        tracing::info!("shutting down data recv stream.");
                        break;
                    }
                }
            }
        });
    }

    #[doc = r" Receives an event from the server"]
    async fn receive(&mut self, event: Event) {
        todo!()
    }

    #[doc = r" Closes the connection agent and its connection to the peer."]
    async fn close(&mut self) {
        todo!()
    }
}
