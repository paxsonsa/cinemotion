use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use api::Property;
use uuid::Uuid;

use crate::api;
use crate::engine::Engine;
use crate::{Error, Result};

#[cfg(test)]
#[path = "./runtime_test.rs"]
mod runtime_test;

/// A runtime loop is a task that runs in the background and executes a tick handler at a given
/// interval. The tick handler is responsible for updating the engine state and notifying the
/// runtime of engine property updates.
struct MotionRuntimeLoop {
    /// The engine instance that is being updated by the runtime loop.
    engine: Arc<Mutex<Box<Engine>>>,

    /// The main loop task handle.
    main_loop: Option<tokio::task::JoinHandle<()>>,

    /// The shutdown channel used to signal the main loop to stop.
    shutdown_channel: Option<tokio::sync::broadcast::Sender<()>>,
}

impl MotionRuntimeLoop {
    /// Create a new RuntimeLoop instance with the given engine.
    fn new(engine: Arc<Mutex<Box<Engine>>>) -> Self {
        Self {
            engine,
            main_loop: None,
            shutdown_channel: None,
        }
    }

    /// Start the runtime loop with the given tick handler and interval.
    ///
    /// The runtime loop will continue to execute until either the tick
    /// handler fails or the `stop()` method is called.
    fn start(
        &mut self,
        interval: tokio::time::Interval,
        tick_handler: fn(&Arc<Mutex<Box<Engine>>>),
    ) {
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);
        self.shutdown_channel = Some(shutdown_tx);
        let engine_mtx = self.engine.clone();
        let mut interval = interval;
        self.main_loop = Some(tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        tick_handler(&engine_mtx);
                    }
                    _ = shutdown_rx.recv() => {
                        break;
                    }
                }
            }
        }));
    }

    /// Stop the runtime loop and await its termination.
    async fn stop(&mut self) -> Result<()> {
        if let Some(shutdown_tx) = self.shutdown_channel.take() {
            shutdown_tx
                .send(())
                .expect("Failed to send shutdown signal to runtime loop");
        }
        if let Some(main_loop) = self.main_loop.take() {
            let _ = main_loop.await;
        }
        Ok(())
    }
}

/// A runtime is responsible for managing all the state of the session and engine.
/// It is the coordinator between the different system controllers and should be used
/// for interacting with the session.
pub struct MotionRuntime<O: MotionRuntimeObserver> {
    state: api::SessionState,
    observer: Option<O>,
    clients: HashMap<Uuid, api::ClientMetadata>,
    engine: Arc<Mutex<Box<Engine>>>,
    main_loop: Option<MotionRuntimeLoop>,
}

impl<O> Default for MotionRuntime<O>
where
    O: MotionRuntimeObserver + 'static + Send + Sync,
{
    fn default() -> Self {
        Self {
            state: api::SessionState::default(),
            observer: None,
            clients: HashMap::new(),
            engine: Arc::new(Mutex::new(Engine::boxed())),
            main_loop: None,
        }
    }
}

impl<O> MotionRuntime<O>
where
    O: MotionRuntimeObserver + 'static + Send + Sync,
{
    /// Create a new Runtime instance with the given observer.
    pub fn new(observer: O) -> Self {
        Self {
            state: api::SessionState::default(),
            observer: Some(observer),
            clients: HashMap::new(),
            engine: Arc::new(Mutex::new(Engine::boxed())),
            main_loop: None,
        }
    }

    /// Set the observer for the runtime.
    pub fn with_observer(&mut self, observer: O) -> &mut Self {
        self.observer = Some(observer);
        self
    }

    /// Add a new client to the runtime and notify the observer.
    ///
    /// This will error is the session mode is `api::SessionMode::Recording` as
    /// clients cannot be added to a recording session once it has started. Move the
    /// session to `api::SessionMode::Idle` or `api::SessionMode::Live` before adding clients.
    ///
    pub async fn add_client(&mut self, client: api::ClientMetadata) -> Result<()> {
        self.clients.insert(client.id, client);

        if let api::SessionMode::Recording = self.state.mode {
            return Err(crate::Error::InvalidRecordingOperation(
                "Cannot add client to runtime when recording",
            ));
        }

        self.report_client_update().await;
        Ok(())
    }

    /// Remove the client with the given id from the runtime and notify the observer.
    ///
    /// Attempting to remove a client while the session is not `api::SessionMode::Idle`
    /// will result in the session being moved to `api::SessionMode::Idle`.
    ///
    /// All properties associated with the client will be removed from the engine.
    pub async fn remove_client(&mut self, id: Uuid) -> Result<()> {
        match self.state.mode {
            api::SessionMode::Recording | api::SessionMode::Live => {
                self.update_mode(api::SessionMode::Idle).await?;
            }
            _ => {}
        }
        self.clients.remove(&id);

        // Clear all properties associated with the client.
        let properties: Vec<Property>;
        {
            let mut engine = self.engine.lock().unwrap();
            while let Some(property) = engine.properties().first() {
                if property.id().namespace == id.to_string() {
                    engine.remove_property(property.id());
                }
            }
            properties = engine.properties();
        }

        self.report_property_update(&properties).await;
        self.report_client_update().await;
        Ok(())
    }

    /// Reports a client list update to the observer.
    async fn report_client_update(&self) {
        let clients = self.clients.values().map(|v| v.to_owned()).collect();
        if let Some(observer) = &self.observer {
            observer.visit_client_update(&clients).await;
        }
    }

    /// Update the current session mode.
    ///
    /// When the session mode is changed to `api::SessionMode::Recording`
    /// or `api::SessionMode::Live`, the main engine loop will be started.
    ///
    /// This will emit property updates to the observer for all properties assigned.
    ///
    /// When the session mode is changed to `api::SessionMode::Idle`, the main engine loop is
    /// killed and the engine state is reset to defaults.
    pub async fn update_mode(&mut self, mode: api::SessionMode) -> Result<()> {
        if self.state.mode.variant_eq(&mode) {
            return Ok(());
        }
        let previous_mode = self.state.mode;
        self.state.mode = mode;

        match (previous_mode, mode) {
            (api::SessionMode::Idle, api::SessionMode::Live)
            | (api::SessionMode::Idle, api::SessionMode::Recording) => match &self.main_loop {
                Some(_) => {}
                None => {
                    let engine_mtx = self.engine.clone();
                    let mut main_loop = MotionRuntimeLoop::new(engine_mtx);
                    let mut interval =
                        tokio::time::interval(std::time::Duration::from_secs_f64(1.0 / 60.0));
                    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
                    main_loop.start(interval, move |mtx| {
                        let mut engine = mtx.lock().unwrap();
                        match engine.step() {
                            Ok(_) => {
                                // TODO: Report Property Updates to clients.
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    });
                    self.main_loop = Some(main_loop);
                }
            },
            (api::SessionMode::Recording, api::SessionMode::Idle)
            | (api::SessionMode::Live, api::SessionMode::Idle) => {
                let Some(mut main_loop) = self.main_loop.take() else {
                    return Err(
                        Error::RuntimeLoopFailed("Failed to stop runtime loop because it was not running.")
                    );
                };

                // TODO: Reset Engine State.
                main_loop.stop().await?;
                let mut engine = self.engine.lock().unwrap();
                engine.reset();
            }
            _ => {}
        }

        if let Some(observer) = &self.observer {
            observer.visit_session_update(&self.state).await;
        }
        Ok(())
    }

    /// Add a new property to the engine.
    ///
    /// New properites cannot be added when the session mode is not `api::SessionMode::Idle`
    /// and a `api::Error::InvalidRecordingOperation` will be returned in this case.
    ///
    pub async fn add_property(&mut self, id: api::ProperyID, default_value: api::PropertyValue) {
        let prop = api::Property::new_with_id(id, default_value);
        // TODO Check Session Mode, Deny when Recording.
        let mut properties = vec![];
        if let Ok(mut engine) = self.engine.lock() {
            engine.add_property(prop);
            properties = engine.properties();
        }

        self.report_property_update(&properties).await;
    }

    pub async fn remove_property(&mut self, id: api::ProperyID) {
        let mut properties = vec![];
        if let Ok(mut engine) = self.engine.lock() {
            engine.remove_property(&id);
            properties = engine.properties();
        }
    }

    /// Update the value of a property.
    ///
    /// The given property value is updated for the given property id. When the session mode is
    /// `api::SessionMode::Idle`, the property value is update but upon the next reversion to `api::SessionMode::Idle`
    /// state, the property value will be reset to the default value.
    ///
    /// Internal Engine errors are propogatd to the caller is the property id is not found or the property
    /// value mismatches the property value type defined.
    ///
    pub async fn update_property(
        &mut self,
        id: api::ProperyID,
        value: api::PropertyValue,
    ) -> Result<()> {
        // TODO Check Session Mode, Ignore when not recording/live.
        if let Ok(mut engine) = self.engine.lock() {
            engine.append_property_update(id, value)?;
        }
        Ok(())
    }

    async fn report_property_update(&self, properties: &Vec<Property>) {
        if let Some(observer) = &self.observer {
            observer.visit_property_update(properties).await;
        }
    }
}

/// An observer for the runtime operations
#[async_trait::async_trait]
pub trait MotionRuntimeObserver {
    /// Called when the client list is updated.
    async fn visit_client_update(&self, clients: &Vec<api::ClientMetadata>);

    /// Called when the session state is updated.
    async fn visit_session_update(&self, state: &api::SessionState);

    /// Called when a property is updated.
    async fn visit_property_update(&self, properties: &Vec<api::Property>);
}
