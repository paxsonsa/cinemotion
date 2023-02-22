use std::collections::HashMap;

use uuid::Uuid;

use crate::{async_trait, AttrName, Attribute, SessionState};

#[derive(Debug, Clone)]
pub enum ClientRole {
    Controller,
    Viewer,
    Renderer
}

#[derive(Debug, Clone)]
pub struct ClientMetadata {
    pub id: Uuid,
    pub name: String,
    pub role: ClientRole
}

impl ClientMetadata {
    /// Create a new ClientMetadata instance with random ID
    pub fn new(name: String, role: ClientRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            role
        }
    }
}



#[derive(Debug)]
pub struct Client {
    pub meta: ClientMetadata,
    pub relay: Box<dyn ClientRelay>
}

impl Client {
    pub fn new<>(name: String, role: ClientRole, relay: Box<dyn ClientRelay>) -> Self {
        Self {
            meta: ClientMetadata::new( name, role),
            relay
        }
    }
}

#[async_trait]
pub trait ClientRelay: std::fmt::Debug + Send + Sync {
    async fn report_client_update(&self, clients: &HashMap<Uuid, ClientMetadata>);
    async fn report_attribute_updates(&self, attributes: &HashMap<AttrName, Attribute>);
    async fn report_session_update(&self, state: &SessionState);
}
