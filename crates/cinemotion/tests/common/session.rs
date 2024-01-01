use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use cinemotion::{
    commands::AddConnection, connection, engine::components::NetworkComponent, name,
    ConnectionAgent, Event, Result,
};

pub struct SpySessionComponent {
    pub create_session_called: bool,
    pub create_session_called_count: usize,
    pub create_session_called_args: Vec<cinemotion::commands::AddConnection>,
    pub close_session_called: bool,
    pub close_session_called_count: usize,
    pub close_session_called_args: Vec<usize>,
    pub send_called: bool,
    pub send_called_count: usize,
    pub send_called_args: Vec<Event>,
}

pub struct FakeSessionComponent {
    pub context: connection::Context,
    pub spy: Arc<Mutex<SpySessionComponent>>,
}

impl FakeSessionComponent {
    pub fn new() -> Self {
        Self {
            context: connection::Context {
                uid: 1,
                name: Some(name!("test")),
            },
            spy: Arc::new(Mutex::new(SpySessionComponent {
                create_session_called: false,
                create_session_called_count: 0,
                create_session_called_args: vec![],
                close_session_called: false,
                close_session_called_count: 0,
                close_session_called_args: vec![],
                send_called: false,
                send_called_count: 0,
                send_called_args: vec![],
            })),
        }
    }
}

#[async_trait]
impl NetworkComponent for FakeSessionComponent {
    fn context_mut(&mut self, conn_id: usize) -> &mut connection::Context {
        &mut self.context
    }

    fn context(&self, conn_id: usize) -> Option<&connection::Context> {
        Some(&self.context)
    }

    async fn add_connection(&mut self, options: AddConnection) -> Result<()> {
        let mut spy = self.spy.lock().unwrap();
        spy.create_session_called = true;
        spy.create_session_called_count += 1;
        spy.create_session_called_args.push(options);
        Ok(())
    }

    async fn close_connection(&mut self, session_id: usize) -> Result<()> {
        let mut spy = self.spy.lock().unwrap();
        spy.close_session_called = true;
        spy.close_session_called_count += 1;
        spy.close_session_called_args.push(session_id);
        Ok(())
    }

    async fn send(&mut self, event: Event) -> Result<()> {
        let mut spy = self.spy.lock().unwrap();
        spy.send_called = true;
        spy.send_called_count += 1;
        spy.send_called_args.push(event);
        Ok(())
    }
}

/// A dummy session agent that does nothing
#[derive(Default)]
pub struct DummyAgent {}

#[async_trait]
impl ConnectionAgent for DummyAgent {
    async fn initialize(&mut self, _send_fn: cinemotion::connection::SendHandlerFn) {}
    async fn receive(&mut self, _event: Event) {}
    async fn close(&mut self) {}
}
