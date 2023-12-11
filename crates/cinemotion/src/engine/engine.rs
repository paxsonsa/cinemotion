use crate::{
    commands::{event_pipe, Command, EventPipeTx, Request},
    session::Session,
    Result,
};

use super::EngineOpt;

pub struct Engine {
    event_pipe: EventPipeTx,
    sessions: Vec<Box<dyn Session + Send>>,
}

impl Engine {
    pub fn new(options: EngineOpt) -> Self {
        let event_pipe = event_pipe();
        Self {
            event_pipe,
            sessions: vec![],
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
                let mut session = create_session.session;
                let ack_pipe = create_session.ack_pipe;

                if let Err(err) = session.initialize(self.event_pipe.subscribe()).await {
                    tracing::error!("failed to new initialize session: {err}");
                    return Ok(());
                }

                self.sessions.push(session);

                if ack_pipe.send(Ok(())).is_err() {
                    tracing::error!(
                        "create ack pipe dropped while creating session, dropping session"
                    );
                    let _ = self.sessions.pop();
                }
                Ok(())
            }
        }
    }
}
