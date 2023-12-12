use crate::commands::{Command, Event, EventPipeRx, Request, RequestPipeTx};
use crate::{Error, Result};

use super::{SendHandlerFn, SessionAgent};

/// Manages a session connection to the runtime.
///
/// Each session manages through a particular agent layer.
/// The agent is in charge of managing the communication to the client.
pub struct Session {
    uid: usize,
    event_pipe: EventPipeRx,
    request_pipe: RequestPipeTx,
    agent: Box<dyn SessionAgent + Send>,
}

impl Session {
    pub fn new(
        uid: usize,
        request_pipe: RequestPipeTx,
        event_pipe: EventPipeRx,
        agent: Box<dyn SessionAgent + Send>,
    ) -> Self {
        let task = tokio::spawn(async {});

        // agent.initialize(;

        Session {
            uid,
            event_pipe,
            request_pipe,
            agent: agent.into(),
        }
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

    pub async fn recieve(event: Event) -> Result<()> {
        Ok(())
    }
}
