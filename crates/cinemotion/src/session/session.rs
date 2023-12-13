use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::commands::{Command, EventPipeRx, Request, RequestPipeTx};
use crate::Error;

use super::{SendHandlerFn, SessionAgent};

/// Manages a session connection to the runtime.
///
/// Each session manages through a particular agent layer.
/// The agent is in charge of managing the communication to the client.
pub struct Session {
    uid: usize,
    task: JoinHandle<()>,
    agent: Mutex<Arc<Box<dyn SessionAgent + Send + Sync>>>,
}

impl Session {
    pub fn new(
        uid: usize,
        request_pipe: RequestPipeTx,
        event_pipe: EventPipeRx,
        mut agent: Box<dyn SessionAgent + Send + Sync>,
    ) -> Self {
        agent.initialize(Self::make_send(uid, request_pipe));

        let agent = Mutex::new(Arc::new(agent));

        let task = tokio::spawn(async move {
            loop {
                let event = match event_pipe.recv().await {
                    Ok(event) => event,
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => return,
                };
                // TODO: Capture receive error and close agent.
                // TODO: Handle Shutdown elegantly.
                let agent = agent.lock().await;
                match event.target {
                    Some(target) if target == uid => agent.receive(event).await,
                    Some(_) => {}
                    None => agent.receive(event).await,
                };
            }
        });

        Session { uid, task, agent }
    }

    fn make_send(uid: usize, request_pipe: RequestPipeTx) -> SendHandlerFn {
        Box::new(move |command: Command| {
            let request = Request::with_command(uid, command);
            if let Err(err) = request_pipe.send(request) {
                let msg = format!(
                    "session {} failed to send request, pipe broken. err={err}",
                    uid
                );

                return Err(Error::SessionFailed(msg));
            }

            Ok(())
        })
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.task.abort();
        self.agent.blocking_lock().close();
    }
}
