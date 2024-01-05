use std::sync::{Arc, Mutex};

pub mod harness;
pub mod session;

pub use harness::*;

pub struct EngineSpy {
    pub session_component: Arc<Mutex<session::SpySessionComponent>>,
}

pub fn make_engine() -> (cinemotion::engine::Builder, EngineSpy) {
    let session_component = Box::new(session::FakeSessionComponent::new());
    let session_spy = session_component.spy.clone();
    let builder = cinemotion::Engine::builder().with_network_component(session_component);
    (
        builder,
        EngineSpy {
            session_component: session_spy,
        },
    )
}
