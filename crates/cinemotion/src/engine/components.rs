use async_trait::async_trait;

use crate::{
    commands::{CreateSession, Event},
    Result,
};

#[async_trait]
pub trait SessionComponent: Send + Sync {
    async fn create_session(&mut self, options: CreateSession) -> Result<()>;
    async fn open_session(&mut self, session_id: usize) -> Result<()>;
    async fn close_session(&mut self, session_id: usize) -> Result<()>;
    async fn send(&mut self, event: Event) -> Result<()>;
}
