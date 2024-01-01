use std::sync::Arc;

use tokio::sync::Mutex;

use super::components::network;
use super::Observer;
use crate::{commands, data, events, Command, Event, Message, Result, State};

pub struct Builder {
    initial_state: Option<State>,
    engine_observer: Option<Arc<Mutex<dyn Observer>>>,
    network_component: Option<Box<dyn network::NetworkComponent>>,
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

    pub fn with_network_component(mut self, component: Box<dyn network::NetworkComponent>) -> Self {
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
    network: Box<dyn network::NetworkComponent>,
}

impl Engine {
    pub fn builder() -> Builder {
        Builder::new()
    }
    /// Apply the given message command to the engine.
    pub async fn apply(&mut self, message: Message) -> Result<()> {
        if let Some(observer) = &self.observer {
            observer.lock().await.on_message(&message);
        }
        let source_id = message.source_id;
        let command = message.command;
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
        self.render().await?;

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

                let context = self.network.context_mut(source_id);
                context.name = Some(peer.name.clone());

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
            commands::ControllerCommand::ChangeMode(mode_change) => {
                self.active_state.mode = mode_change.0;
                Ok(())
            }
            commands::ControllerCommand::SampleMotion(sample) => {
                let sample = sample.0;
                // Extract the context from the network item and then get a ref to the name
                let Some(context) = self.network.context(source_id) else {
                    tracing::error!("context not found for id: {}", source_id);
                    return Ok(());
                };

                let Some(name) = context.name.as_ref() else {
                    tracing::error!("no name is assigned to the connection {source_id}");
                    return Ok(());
                };

                let Some(controller) = self.active_state.controllers.get_mut(&name) else {
                    tracing::error!("controller not found for name: {}", name);
                    return Ok(());
                };

                for (property_name, value) in sample.properties() {
                    let Some(property) = controller.properties.get_mut(property_name) else {
                        tracing::error!(
                            "property not found for name: {}.{}",
                            name.to_string(),
                            property_name
                        );
                        continue;
                    };

                    if let Err(err) = property.update(value) {
                        tracing::error!(
                            "error updating property: {}.{}: {}",
                            name.to_string(),
                            property_name,
                            err
                        );
                    }
                }

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
    async fn render(&mut self) -> Result<()> {
        for obj in self.active_state.scene.objects_mut().values_mut() {
            let obj_name = obj.name().clone();
            for (name, property) in obj.properties_mut() {
                match property {
                    data::PropertyLink::Bound { value, binding } => {
                        let Some(controller) =
                            self.active_state.controllers.get(&binding.namespace)
                        else {
                            tracing::error!(
                                "controller not found for name: {} for scene objext property {}.{}",
                                binding.namespace,
                                obj_name,
                                name
                            );
                            continue;
                        };
                        let Some(controller_property) =
                            controller.properties.get(&binding.property)
                        else {
                            tracing::error!(
                                "property not found for name: {}.{}",
                                binding.namespace.to_string(),
                                binding.property
                            );
                            continue;
                        };
                        if let Err(err) = value.update(&controller_property.value) {
                            tracing::error!(
                                "error updating property: {}.{}: {}",
                                obj_name.to_string(),
                                name,
                                err
                            );
                        }
                    }
                    data::PropertyLink::Unbound { .. } => {
                        tracing::error!(
                            "ignored unbound property on scene object {}.{}",
                            obj_name,
                            name
                        );
                    }
                }
            }
        }
        Ok(())
    }
}
