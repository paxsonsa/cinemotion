use std::sync::{Arc, Mutex};

pub mod session;

pub struct EngineSpy {
    pub session_component: Arc<Mutex<session::SpySessionComponent>>,
}

pub fn make_engine() -> (cinemotion::Engine, EngineSpy) {
    let session_component = Box::new(session::FakeSessionComponent::new());
    let session_spy = session_component.spy.clone();
    let engine = cinemotion::Engine::builder()
        .with_session_component(session_component)
        .build()
        .unwrap();
    (
        engine,
        EngineSpy {
            session_component: session_spy,
        },
    )
}
