use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use cinemotion::commands::{CreateSession, Event};
use cinemotion::engine::components::SessionComponent;
use cinemotion::Result;

pub struct SpySessionComponent {
    pub create_session_called: bool,
    pub create_session_called_count: usize,
    pub create_session_called_args: Vec<cinemotion::commands::CreateSession>,
    pub open_session_called: bool,
    pub open_session_called_count: usize,
    pub open_session_called_args: Vec<usize>,
    pub close_session_called: bool,
    pub close_session_called_count: usize,
    pub close_session_called_args: Vec<usize>,
    pub send_called: bool,
    pub send_called_count: usize,
    pub send_called_args: Vec<Event>,
}

pub struct FakeSessionComponent {
    pub spy: Arc<Mutex<SpySessionComponent>>,
}

impl FakeSessionComponent {
    pub fn new() -> Self {
        Self {
            spy: Arc::new(Mutex::new(SpySessionComponent {
                create_session_called: false,
                create_session_called_count: 0,
                create_session_called_args: vec![],
                open_session_called: false,
                open_session_called_count: 0,
                open_session_called_args: vec![],
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
impl SessionComponent for FakeSessionComponent {
    async fn create_session(&mut self, options: CreateSession) -> Result<()> {
        let mut spy = self.spy.lock().unwrap();
        spy.create_session_called = true;
        spy.create_session_called_count += 1;
        spy.create_session_called_args.push(options);
        Ok(())
    }

    async fn open_session(&mut self, session_id: usize) -> Result<()> {
        let mut spy = self.spy.lock().unwrap();
        spy.open_session_called = true;
        spy.open_session_called_count += 1;
        spy.open_session_called_args.push(session_id);
        Ok(())
    }

    async fn close_session(&mut self, session_id: usize) -> Result<()> {
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
