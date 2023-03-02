use std::collections::HashMap;

use uuid::Uuid;

use crate::{async_trait, AttrName, Attribute, SessionState};

#[derive(Debug, Clone)]
pub enum ClientRole {
    PrimaryController,
    SecondaryController,
    Observer,
    Renderer,
}
impl Default for ClientRole {
    fn default() -> Self {
        Self::PrimaryController
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClientMetadata {
    pub id: Uuid,
    pub name: String,
    pub role: ClientRole,
}

impl ClientMetadata {
    /// Create a new ClientMetadata instance with random ID
    pub fn new(name: String, role: ClientRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            role,
        }
    }
}

#[derive(Default, Debug)]
pub struct Client {
    pub meta: ClientMetadata,
    pub relay: Option<Box<dyn ClientRelay>>,
}

impl Client {
    pub fn new(name: String, role: ClientRole) -> Self {
        Self {
            meta: ClientMetadata::new(name, role),
            relay: None,
        }
    }

    pub fn with_relay(mut self, relay: Box<dyn ClientRelay>) -> Self {
        self.relay = Some(relay);
        self
    }
}

#[async_trait]
pub trait ClientRelay: std::fmt::Debug + Send + Sync {}
