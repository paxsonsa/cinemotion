use std::collections::HashMap;

use super::*;
use crate::{engine::components::network::NetworkComponent, *};

struct NetworkSpyValues {
    events: Vec<Event>,
}

impl NetworkSpyValues {
    fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self { events: Vec::new() }))
    }
}

struct NetworkSpy {
    context: connection::Context,
    values: Arc<Mutex<NetworkSpyValues>>,
}

impl NetworkSpy {
    fn new(values: Arc<Mutex<NetworkSpyValues>>) -> Self {
        Self {
            context: Default::default(),
            values,
        }
    }
}

#[async_trait::async_trait]
impl super::network::NetworkComponent for NetworkSpy {
    // Get a mutable reference to the context for a connection id.
    fn context_mut(&mut self, _: usize) -> &mut connection::Context {
        &mut self.context
    }
    /// Get the context for a connection id.
    fn context(&self, _: usize) -> Option<&connection::Context> {
        Some(&self.context)
    }
    /// Create and add a connection to manage
    async fn add_connection(&mut self, _: commands::AddConnection) -> Result<()> {
        Ok(())
    }
    /// Close the connection and stop communicating with the peer.
    async fn close_connection(&mut self, _: usize) -> Result<()> {
        Ok(())
    }
    /// Send an event to the connected peers.
    async fn send(&mut self, event: Event) -> Result<()> {
        self.values.lock().await.events.push(event);
        Ok(())
    }
}

#[tokio::test]
async fn test_init_errors_when_not_idle() {
    let state = State {
        mode: crate::data::Mode::Live,
        ..Default::default()
    };

    let values = NetworkSpyValues::new();
    let network = Box::new(NetworkSpy::new(values.clone()));
    let mut engine = Engine::builder()
        .with_inital_state(state)
        .with_network_component(network)
        .build()
        .expect("failed to build engine");

    let command = crate::commands::Init {
        peer: data::Controller {
            name: name!("controllerA"),
            properties: HashMap::new(),
        },
    };
    assert!(matches!(
        engine.handle_init(command, 1),
        Err(Error::InvalidMode(_))
    ));
}

#[tokio::test]
async fn test_update_scene_obj_errors_when_not_idle() {
    let state = State {
        mode: crate::data::Mode::Live,
        ..Default::default()
    };

    let values = NetworkSpyValues::new();
    let network = Box::new(NetworkSpy::new(values.clone()));
    let mut engine = Engine::builder()
        .with_inital_state(state)
        .with_network_component(network)
        .build()
        .expect("failed to build engine");

    let command =
        crate::commands::UpdateSceneObject(crate::SceneObject::new(name!("obj"), HashMap::new()));

    assert!(matches!(
        engine.handle_update_scene_obj(command),
        Err(Error::InvalidMode(_))
    ));
}

#[tokio::test]
async fn test_add_scene_obj_errors_when_not_idle() {
    let state = State {
        mode: crate::data::Mode::Live,
        ..Default::default()
    };

    let values = NetworkSpyValues::new();
    let network = Box::new(NetworkSpy::new(values.clone()));
    let mut engine = Engine::builder()
        .with_inital_state(state)
        .with_network_component(network)
        .build()
        .expect("failed to build engine");

    let command =
        crate::commands::AddSceneObject(crate::SceneObject::new(name!("obj"), HashMap::new()));

    assert!(matches!(
        engine.handle_add_scene_obj(command),
        Err(Error::InvalidMode(_))
    ));
}

#[tokio::test]
async fn test_delete_scene_obj_errors_when_not_idle() {
    let state = State {
        mode: crate::data::Mode::Live,
        ..Default::default()
    };

    let values = NetworkSpyValues::new();
    let network = Box::new(NetworkSpy::new(values.clone()));
    let mut engine = Engine::builder()
        .with_inital_state(state)
        .with_network_component(network)
        .build()
        .expect("failed to build engine");

    let command = crate::commands::DeleteSceneObject(name!("objectA"));

    assert!(matches!(
        engine.handle_delete_scene_obj(command),
        Err(Error::InvalidMode(_))
    ));
}
