

use super::*;

struct MockRuntimeObserver {
    client_update_handler: Option<fn(&Vec<api::ClientMetadata>)>,
    session_update_handler: Option<fn(&api::SessionState)>,
    property_update_handler: Option<fn(&Vec<api::Property>)>,
}

impl MockRuntimeObserver {
    fn new() -> MockRuntimeObserver {
        Self {
            client_update_handler: None,
            session_update_handler: None,
            property_update_handler: None,
        }
    }

    fn with_client_update_handler(&mut self, handler: fn(&Vec<api::ClientMetadata>)) -> &mut Self {
        self.client_update_handler = Some(handler);
        self
    }

    fn with_session_update_handler(&mut self, handler: fn(&api::SessionState)) -> &mut Self {
        self.session_update_handler = Some(handler);
        self
    }

    fn with_property_update(&mut self, handler: fn(&Vec<api::Property>)) -> &mut Self {
        self.property_update_handler = Some(handler);
        self
    }
}

#[async_trait::async_trait]
impl MotionRuntimeObserver for MockRuntimeObserver {
    async fn visit_client_update(&self, clients: &Vec<api::ClientMetadata>) {
        if let Some(func) = self.client_update_handler {
            func(clients);
        }
    }

    async fn visit_session_update(&self, state: &api::SessionState) {
        if let Some(func) = self.session_update_handler {
            func(state);
        }
    }

    async fn visit_property_update(&self, properties: &Vec<api::Property>) {
        if let Some(func) = self.property_update_handler {
            func(properties);
        }
    }
}

#[tokio::test]
async fn test_adding_removing_client_to_runtime() {
    let mut observer = MockRuntimeObserver::new();
    observer.with_client_update_handler(|clients| {
        assert!(clients.len() == 1 || clients.is_empty());
    });
    let mut runtime = MotionRuntime::new(observer);

    let client = api::ClientMetadata::new("Test Client".to_string(), api::ClientRole::Controller);

    runtime
        .add_client(client.clone())
        .await
        .expect("Failed to add client to runtime");
    assert_eq!(runtime.clients.len(), 1);

    runtime
        .remove_client(client.id)
        .await
        .expect("Failed to remove client from runtime");
    assert_eq!(runtime.clients.len(), 0);
}

/// Adding a client while recording should fail.
#[tokio::test]
async fn test_adding_client_to_runtime_when_recording_fails() {
    let observer = MockRuntimeObserver::new();
    let mut runtime = MotionRuntime::new(observer);
    runtime
        .update_mode(api::SessionMode::Recording)
        .await
        .expect("Failed to update runtime mode");

    let client = api::ClientMetadata::new("Test Client".to_string(), api::ClientRole::Controller);
    assert!(
        matches!(
            runtime.add_client(client).await,
            Err(crate::Error::InvalidRecordingOperation(_))
        ),
        "Adding client to runtime when recording should fail"
    );
}

/// Removing a client while recording should move mode to idle if recording or live.
#[tokio::test]
async fn test_remove_client_to_runtime_when_recording() {
    let observer = MockRuntimeObserver::new();
    let mut runtime = MotionRuntime::new(observer);
    let client = api::ClientMetadata::new("Test Client".to_string(), api::ClientRole::Controller);
    runtime
        .add_client(client.clone())
        .await
        .expect("Failed to add client to runtime");

    runtime
        .update_mode(api::SessionMode::Recording)
        .await
        .expect("Failed to update runtime mode");
    runtime
        .remove_client(client.id)
        .await
        .expect("Failed to remove client from runtime");

    assert_eq!(runtime.state.mode, api::SessionMode::Idle);
}

#[tokio::test]
async fn test_updating_mode() {
    let observer = MockRuntimeObserver::new();
    let mut runtime = MotionRuntime::new(observer);

    assert_eq!(runtime.state.mode, api::SessionMode::Idle);
    runtime
        .update_mode(api::SessionMode::Live)
        .await
        .expect("Failed to update runtime mode");

    assert_eq!(runtime.state.mode, api::SessionMode::Live);
    assert!(runtime.main_loop.is_some());

    runtime
        .update_mode(api::SessionMode::Recording)
        .await
        .expect("Failed to update runtime mode");
    assert!(runtime.main_loop.is_some());
    assert_eq!(runtime.state.mode, api::SessionMode::Recording);

    runtime
        .update_mode(api::SessionMode::Idle)
        .await
        .expect("Failed to update runtime mode");
    assert_eq!(runtime.state.mode, api::SessionMode::Idle);
    assert!(runtime.main_loop.is_none());
}

#[tokio::test]
async fn test_updating_mode_triggers_observer() {
    let mut observer = MockRuntimeObserver::new();
    observer.with_session_update_handler(|state| {
        assert_eq!(state.mode, api::SessionMode::Live);
    });
    let mut runtime = MotionRuntime::new(observer);

    assert_eq!(runtime.state.mode, api::SessionMode::Idle);
    runtime
        .update_mode(api::SessionMode::Live)
        .await
        .expect("Failed to update runtime mode");
}
