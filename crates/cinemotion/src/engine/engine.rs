use std::sync::Arc;

use tokio::sync::Mutex;

use super::components::NetworkComponent;
use super::Observer;
use crate::{commands, events, Command, Event, Message, Result, State};

pub struct Builder {
    initial_state: Option<State>,
    engine_observer: Option<Arc<Mutex<dyn Observer>>>,
    network_component: Option<Box<dyn NetworkComponent>>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            initial_state: None,
            engine_observer: None,
            network_component: None,
        }
    }
    pub fn with_inital_state(mut self, state: State) -> Self {
        self.initial_state = Some(state);
        self
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
        let state = self.initial_state.unwrap_or_default();
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
        let result = match command {
            Command::Controller(client_command) => {
                self.handle_client_command(source_id, client_command).await
            }
            Command::System(internal_command) => {
                self.handle_internal_command(source_id, internal_command)
                    .await
            }
        };
        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("error applying message: {}", err);
                self.send(Event {
                    target: Some(source_id),
                    body: events::ErrorEvent(err).into(),
                })
                .await?;
                Ok(())
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
        client_command: crate::commands::ControllerCommand,
    ) -> Result<()> {
        match client_command {
            commands::ControllerCommand::Echo(message) => {
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
            commands::ControllerCommand::Init(init) => {
                let peer = init.peer;
                self.active_state
                    .controllers
                    .insert(peer.name.clone(), peer);
                Ok(())
            }
            commands::ControllerCommand::UpdateSceneObject(update) => {
                let scene_object = update.0;
                let name = scene_object.name().clone();
                let objects = self.active_state.scene.objects_mut();
                match objects.contains_key(&name) {
                    true => {
                        objects.insert(name, scene_object);
                        Ok(())
                    }
                    false => Err(crate::Error::InvalidSceneObject(
                        "object does not exist".into(),
                    )),
                }
            }
            commands::ControllerCommand::AddSceneObject(object) => {
                let object = object.0;
                let name = object.name().clone();
                let objects = self.active_state.scene.objects_mut();
                match objects.contains_key(&name) {
                    true => Err(crate::Error::InvalidSceneObject(
                        "object already exists".into(),
                    )),
                    false => {
                        objects.insert(name, object);
                        Ok(())
                    }
                }
            }
            commands::ControllerCommand::DeleteSceneObject(name) => {
                self.active_state.scene.objects_mut().remove(&name.0);
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
                    body: events::ConnectionOpenedEvent().into(),
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
