use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    commands::{event_pipe, CreateSession, Event, EventPipeTx, RequestPipeTx},
    session::Session,
    Error, Result,
};

use super::components::SessionComponent;

pub struct SessionComponentImpl {
    sessions: HashMap<usize, Box<Session>>,
    request_pipe: RequestPipeTx,
    event_pipe: EventPipeTx,
}

impl SessionComponentImpl {
    pub fn boxed(request_pipe: RequestPipeTx) -> Box<dyn SessionComponent> {
        let event_pipe = event_pipe();
        Box::new(Self {
            sessions: Default::default(),
            request_pipe,
            event_pipe,
        })
    }
}

#[async_trait]
impl SessionComponent for SessionComponentImpl {
    async fn create_session(&mut self, options: CreateSession) -> Result<()> {
        let agent = options.agent;
        let ack_pipe = options.ack_pipe;
        let active_id = self.sessions.len() + 1;
        let session = Box::new(Session::new(
            active_id,
            self.request_pipe.clone(),
            self.event_pipe.subscribe(),
            agent,
        ));
        self.sessions.insert(active_id, session);
        if ack_pipe.send(Ok(())).is_err() {
            tracing::error!("create ack pipe dropped while creating session, dropping session");
            let _ = self.sessions.remove(&active_id);
        }
        Ok(())
    }

    async fn open_session(&mut self, session_id: usize) -> Result<()> {
        todo!()
    }

    async fn close_session(&mut self, session_id: usize) -> Result<()> {
        todo!()
    }

    async fn send(&mut self, event: Event) -> Result<()> {
        if self.event_pipe.send(event).is_err() {
            return Err(Error::EngineFailed(
                "event pipe closed unexpectedly.".to_string(),
            ));
        }
        Ok(())
    }
}
