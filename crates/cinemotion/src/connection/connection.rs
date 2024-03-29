use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::events::EventPipeRx;
use crate::messages::{Message, MessagePipeTx, Payload};
use crate::Error;

use super::{ConnectionAgent, SendHandlerFn};

/// Manages a connection to the runtime.
///
/// Each connection manages through a particular agent layer.
/// The agent is in charge of managing the communication to the client.
pub struct Connection {
    uid: usize,
    task: JoinHandle<()>,
    agent: Arc<Mutex<Box<dyn ConnectionAgent + Send + Sync>>>,
}

impl Connection {
    pub fn new(
        uid: usize,
        message_pipe: MessagePipeTx,
        mut event_pipe: EventPipeRx,
        mut agent: Box<dyn ConnectionAgent + Send + Sync>,
    ) -> Self {
        agent.initialize(Self::make_send(uid, message_pipe));

        let agent = Arc::new(Mutex::new(agent));
        let shared_agent = Arc::clone(&agent);
        let task = tokio::spawn(async move {
            loop {
                let event = match event_pipe.recv().await {
                    Ok(event) => event,
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => return,
                };
                // TODO: Capture receive error and close agent.
                // TODO: Handle Shutdown elegantly.
                let mut agent = shared_agent.lock().await;
                match event.target {
                    Some(target) if target == uid => agent.receive(event).await,
                    Some(_) => {}
                    None => agent.receive(event).await,
                };
            }
        });

        Connection { uid, task, agent }
    }

    fn make_send(uid: usize, message_pipe: MessagePipeTx) -> SendHandlerFn {
        Box::new(move |command: Payload| {
            let message = Message::with_command(uid, command);
            if let Err(err) = message_pipe.send(message) {
                let msg = format!(
                    "connection {} failed to send message, pipe broken. err={err}",
                    uid
                );

                return Err(Error::ConnectionFailed(msg));
            }

            Ok(())
        })
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.task.abort();
        self.agent.blocking_lock().close();
    }
}
