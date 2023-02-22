use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::api;
use crate::engine::Engine;
use crate::{Error, Result};

#[cfg(test)]
#[path = "./runtime_test.rs"]
mod runtime_test;

struct RuntimeLoop {
    engine: Arc<Mutex<Box<Engine>>>,
    main_loop: Option<tokio::task::JoinHandle<()>>,
    shutdown_channel: Option<tokio::sync::broadcast::Sender<()>>,
}

impl RuntimeLoop {
    fn new(engine: Arc<Mutex<Box<Engine>>>) -> Self {
        Self {
            engine,
            main_loop: None,
            shutdown_channel: None,
        }
    }

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

pub struct Runtime<O: RuntimeObserver> {
    state: api::SessionState,
    observer: Option<O>,
    clients: HashMap<Uuid, api::ClientMetadata>,
    engine: Arc<Mutex<Box<Engine>>>,
    main_loop: Option<RuntimeLoop>,
}

impl<O> Default for Runtime<O>
where
    O: RuntimeObserver + 'static + Send + Sync,
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

impl<O> Runtime<O>
where
    O: RuntimeObserver + 'static + Send + Sync,
{
    pub fn new(observer: O) -> Self {
        Self {
            state: api::SessionState::default(),
            observer: Some(observer),
            clients: HashMap::new(),
            engine: Arc::new(Mutex::new(Engine::boxed())),
            main_loop: None,
        }
    }

    pub fn with_observer(&mut self, observer: O) -> &mut Self {
        self.observer = Some(observer);
        self
    }

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

    pub async fn remove_client(&mut self, id: Uuid) -> Result<()> {
        match self.state.mode {
            api::SessionMode::Recording | api::SessionMode::Live => {
                self.update_mode(api::SessionMode::Idle).await?;
            }
            _ => {}
        }
        self.clients.remove(&id);
        // TODO Handle when last Controller is removed.
        // - Needs to kill the main loop.
        // - Remove Properties namespace to that client.
        self.report_client_update().await;
        Ok(())
    }

    async fn report_client_update(&self) {
        let clients = self.clients.values().map(|v| v.to_owned()).collect();
        if let Some(observer) = &self.observer {
            observer.visit_client_update(&clients).await;
        }
    }

    /// Update the current session mode.
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
                    let mut main_loop = RuntimeLoop::new(engine_mtx);
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
                // let mut engine = self.engine.lock().unwrap();
                // engine.reset();
            }
            _ => {}
        }

        if let Some(observer) = &self.observer {
            observer.visit_session_update(&self.state).await;
        }
        Ok(())
    }

    pub async fn add_property(&mut self, id: api::ProperyID, default_value: api::PropertyValue) {
        let prop = api::Property::new_with_id(id, default_value);
        // TODO Check Session Mode, Deny when Recording.
        if let Ok(mut engine) = self.engine.lock() {
            engine.add_property(prop);
        }
    }

    pub async fn update_property(&mut self, id: api::ProperyID, value: api::PropertyValue) {
        // TODO Check Session Mode, Ignore when not recording/live.
        if let Ok(mut engine) = self.engine.lock() {
            engine.append_property_update(id, value);
        }
    }
}

#[async_trait::async_trait]
pub trait RuntimeObserver {
    async fn visit_client_update(&self, clients: &Vec<api::ClientMetadata>);
    async fn visit_session_update(&self, state: &api::SessionState);
}
