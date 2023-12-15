use std::sync::Arc;

use tokio::sync::Mutex;

use super::Observer;

use super::components::SessionComponent;
use crate::{commands::*, Result};

pub struct Builder {
    engine_observer: Option<Arc<Mutex<dyn Observer>>>,
    session_component: Option<Box<dyn SessionComponent>>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            engine_observer: None,
            session_component: None,
        }
    }
    pub fn with_engine_observer(mut self, engine_observer: Arc<Mutex<dyn Observer>>) -> Self {
        self.engine_observer = Some(engine_observer);
        self
    }

    pub fn with_session_component(mut self, session_component: Box<dyn SessionComponent>) -> Self {
        self.session_component = Some(session_component);
        self
    }

    pub fn build(self) -> Result<Engine> {
        let session_component = self.session_component.unwrap();
        let observer = self.engine_observer;
        Ok(Engine {
            observer,
            session_component,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Engine {
    observer: Option<Arc<Mutex<dyn Observer>>>,
    session_component: Box<dyn SessionComponent>,
}

impl Engine {
    pub fn builder() -> Builder {
        Builder::new()
    }
    /// Apply the given request command to the engine.
    pub async fn apply(&mut self, request: Request) -> Result<()> {
        if let Some(observer) = &self.observer {
            observer.lock().await.on_request(&request);
        }
        let source_id = request.session_id;
        let command = request.command;
        match command {
            Command::Client(client_command) => {
                self.handle_client_command(source_id, client_command).await
            }
            Command::Internal(internal_command) => {
                self.handle_internal_command(internal_command).await
            }
        }
    }

    pub async fn tick(&mut self) -> Result<()> {
        Ok(())
    }

    async fn handle_client_command(
        &mut self,
        source_id: usize,
        client_command: crate::commands::ClientCommand,
    ) -> Result<()> {
        match client_command {
            ClientCommand::Echo(message) => {
                tracing::info!("echo: {message}");
                let event = Event {
                    target: Some(source_id),
                    payload: EventPayload::Echo(message),
                };
                if let Some(observer) = &self.observer {
                    observer.lock().await.on_event(&event);
                }
                self.session_component.send(event).await?;
                Ok(())
            }
        }
    }

    async fn handle_internal_command(
        &mut self,
        internal_command: crate::commands::InternalCommand,
    ) -> Result<()> {
        match internal_command {
            InternalCommand::CreateSession(session) => {
                self.session_component.create_session(session).await?;
                Ok(())
            }
            InternalCommand::OpenSession(open_session) => Ok(()),
        }
    }
}
