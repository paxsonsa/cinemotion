use std::sync::Arc;

use tokio::sync::Mutex;

use super::components::network;
use super::Observer;
use crate::{data, events, messages, Event, Result, State};

#[cfg(test)]
#[path = "engine_test.rs"]
mod engine_test;

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
        let network = self
            .network_component
            .expect("expect network component to be supplied");

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
    pub async fn apply(&mut self, message: messages::Message) -> Result<()> {
        if let Some(observer) = &self.observer {
            observer.lock().await.on_message(&message);
        }
        let source_id = message.source_id;
        let command = message.command;
        let result = match command {
            messages::Payload::Client(client_command) => {
                self.handle_client_command(source_id, client_command).await
            }
            messages::Payload::System(internal_command) => {
                self.handle_internal_command(source_id, internal_command)
                    .await
            }
            messages::Payload::Invalid => {
                tracing::error!("invalid command received from {}", source_id);
                Ok(())
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
        client_command: crate::messages::ClientCommand,
    ) -> Result<()> {
        match client_command {
            messages::ClientCommand::Echo(message) => self.handle_echo(source_id, message).await,
            messages::ClientCommand::Init(init) => self.handle_init(init, source_id),
            messages::ClientCommand::UpdateSceneObject(update) => {
                self.handle_update_scene_obj(update)
            }
            messages::ClientCommand::AddSceneObject(object) => self.handle_add_scene_obj(object),
            messages::ClientCommand::ClearScene(_) => {
                self.ensure_idle_mode()?;
                self.active_state.scene.objects_mut().clear();
                Ok(())
            }
            messages::ClientCommand::DeleteSceneObject(name) => self.handle_delete_scene_obj(name),
            messages::ClientCommand::ChangeMode(mode_change) => {
                self.handle_mode_change(mode_change)
            }
            messages::ClientCommand::SampleMotion(sample) => self.handle_sample(sample, source_id),
        }
    }

    fn handle_sample(&mut self, sample: messages::SampleMotion, source_id: usize) -> Result<()> {
        if self.active_state.mode.is_idle() {
            tracing::debug!("ignoring sample motion command because the mode is idle");
            return Ok(());
        }
        let sample = sample.0;
        let Some(context) = self.network.context(source_id) else {
            tracing::error!("context not found for id: {}", source_id);
            return Ok(());
        };
        let Some(name) = context.name.as_ref() else {
            tracing::error!("no name is assigned to the connection {source_id}");
            return Ok(());
        };
        let Some(controller) = self.active_state.controllers.get_mut(name) else {
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

    fn handle_mode_change(&mut self, mode_change: messages::ChangeMode) -> Result<()> {
        let is_sample_mode = !self.active_state.mode.is_idle();
        if is_sample_mode && mode_change.0.is_idle() {
            // Reset the sampling state, we don't need to worry about the scene objects
            // because the will be updated when the engine renders.
            reset_controller_properties(&mut self.active_state);
        }
        self.active_state.mode = mode_change.0;
        Ok(())
    }

    fn handle_delete_scene_obj(&mut self, name: messages::DeleteSceneObject) -> Result<()> {
        self.ensure_idle_mode()?;
        self.active_state.scene.objects_mut().remove(&name.0);
        Ok(())
    }

    fn handle_add_scene_obj(&mut self, object: messages::AddSceneObject) -> Result<()> {
        self.ensure_idle_mode()?;
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

    fn handle_update_scene_obj(&mut self, update: messages::UpdateSceneObject) -> Result<()> {
        self.ensure_idle_mode()?;
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

    fn handle_init(&mut self, init: messages::Init, source_id: usize) -> Result<()> {
        self.ensure_idle_mode()?;
        let peer = init.peer;
        let context = self.network.context_mut(source_id);
        context.name = Some(peer.name.clone());
        self.active_state
            .controllers
            .insert(peer.name.clone(), peer);
        Ok(())
    }

    async fn handle_echo(&mut self, source_id: usize, message: messages::Echo) -> Result<()> {
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

    async fn handle_internal_command(
        &mut self,
        source_id: usize,
        internal_command: crate::messages::SystemCommand,
    ) -> Result<()> {
        match internal_command {
            messages::SystemCommand::AddConnection(conn) => {
                self.network.add_connection(conn).await?;
                Ok(())
            }
            messages::SystemCommand::OpenConnection(_) => {
                self.send(Event {
                    target: Some(source_id),
                    body: events::ConnectionOpenedEvent().into(),
                })
                .await
            }
            messages::SystemCommand::CloseConnection(_) => {
                self.network.close_connection(source_id).await
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

    fn ensure_idle_mode(&self) -> Result<()> {
        if self.active_state.mode.is_live() {
            Err(crate::Error::InvalidMode(
                "cannot perform command while in live mode".into(),
            ))
        } else {
            Ok(())
        }
    }
}

// Reset the sampling state of the given state.
pub fn reset_controller_properties(state: &mut State) {
    for (_, controller) in state.controllers.iter_mut() {
        for (_, property) in controller.properties.iter_mut() {
            if let Err(err) = property.reset() {
                tracing::error!("error resetting property: {}", err);
            }
        }
    }
}
