use std::sync::Arc;

use tokio::sync::Mutex;

use super::{Observer, State};

use super::components::NetworkComponent;
use crate::{commands, events, Command, Event, Message, Result};

pub struct Builder {
    engine_observer: Option<Arc<Mutex<dyn Observer>>>,
    network_component: Option<Box<dyn NetworkComponent>>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            engine_observer: None,
            network_component: None,
        }
    }
    pub fn with_engine_observer(mut self, engine_observer: Arc<Mutex<dyn Observer>>) -> Self {
        self.engine_observer = Some(engine_observer);
        self
    }

    pub fn with_network_component(mut self, component: Box<dyn NetworkComponent>) -> Self {
        self.network_component = Some(component);
        self
    }

    pub fn build(self) -> Result<Engine> {
        let state = State::default();
        let network = self.network_component.unwrap();
        let observer = self.engine_observer;
        Ok(Engine {
            active_state: state.clone(),
            current_state: state,
            observer,
            network,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Engine {
    active_state: State,
    current_state: State,
    observer: Option<Arc<Mutex<dyn Observer>>>,
    network: Box<dyn NetworkComponent>,
}

impl Engine {
    pub fn builder() -> Builder {
        Builder::new()
    }
    /// Apply the given request command to the engine.
    pub async fn apply(&mut self, request: Message) -> Result<()> {
        if let Some(observer) = &self.observer {
            observer.lock().await.on_request(&request);
        }
        let source_id = request.source_id;
        let command = request.command;
        match command {
            Command::Peer(client_command) => {
                self.handle_client_command(source_id, client_command).await
            }
            Command::System(internal_command) => {
                self.handle_internal_command(source_id, internal_command)
                    .await
            }
        }
    }

    pub async fn tick(&mut self) -> Result<()> {
        if let Some(observer) = &self.observer {
            observer.lock().await.on_state_change(&self.active_state);
        }
        self.current_state = self.active_state.clone();
        self.send(Event {
            target: None,
            body: events::StateChangeEvent(self.current_state.clone()).into(),
        })
        .await
    }

    async fn handle_client_command(
        &mut self,
        source_id: usize,
        client_command: crate::commands::PeerCommand,
    ) -> Result<()> {
        match client_command {
            commands::PeerCommand::Echo(message) => {
                tracing::info!("echo: {message}");
                let event = Event {
                    target: Some(source_id),
                    body: events::EventBody::Echo(message),
                };
                if let Some(observer) = &self.observer {
                    observer.lock().await.on_event(&event);
                }
                self.send(event).await?;
                Ok(())
            }
            commands::PeerCommand::Init(init) => {
                let peer = init.peer;
                self.active_state.peers.push(peer);
                Ok(())
            }
        }
    }

    async fn handle_internal_command(
        &mut self,
        source_id: usize,
        internal_command: crate::commands::SystemCommand,
    ) -> Result<()> {
        match internal_command {
            commands::SystemCommand::AddConnection(conn) => {
                self.network.add_connection(conn).await?;
                Ok(())
            }
            commands::SystemCommand::OpenConnection(_) => {
                self.send(Event {
                    target: Some(source_id),
                    body: events::ConnectionOpenedEvent {}.into(),
                })
                .await
            }
        }
    }

    async fn send(&mut self, event: Event) -> Result<()> {
        if let Some(observer) = &self.observer {
            observer.lock().await.on_event(&event);
        }
        self.network.send(event).await
    }
}
