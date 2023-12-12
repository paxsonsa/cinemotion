use std::collections::HashMap;

use crate::{
    commands::{event_pipe, Command, EventPipeTx, Request, RequestPipeTx},
    session::{Session, SessionAgent, LOCAL_SESSION_ID},
    Result,
};

use super::EngineOpt;

#[cfg(test)]
#[path = "./engine_test.rs"]
mod engine_test;

pub struct Engine {
    request_pipe: RequestPipeTx,
    event_pipe: EventPipeTx,
    sessions: HashMap<usize, Box<Session>>,
}

impl Engine {
    pub fn new(options: EngineOpt) -> Self {
        let event_pipe = options.event_pipe;
        let request_pipe = options.request_pipe;
        Self {
            request_pipe,
            event_pipe,
            sessions: Default::default(),
        }
    }
    pub async fn apply(&mut self, request: Request) -> Result<()> {
        let command = request.command;
        match command {
            Command::Echo(message) => {
                tracing::info!("echo: {message}");
                Ok(())
            }
            Command::CreateSession(create_session) => {
                let agent = create_session.agent;
                let ack_pipe = create_session.ack_pipe;
                let active_id = self.sessions.len() + 1;

                let session = Box::new(Session::new(
                    active_id,
                    self.request_pipe.clone(),
                    self.event_pipe.subscribe(),
                    agent,
                ));

                self.sessions.insert(active_id, session);

                if ack_pipe.send(Ok(())).is_err() {
                    tracing::error!(
                        "create ack pipe dropped while creating session, dropping session"
                    );
                    let _ = self.sessions.remove(&active_id);
                }
                Ok(())
            }
        }
    }
}
