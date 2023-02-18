use std::collections::HashMap;

use uuid::Uuid;

use crate::{AttrName, Attribute, SessionState};

#[derive(Debug, Clone)]
pub enum ClientRole {
    Controller,
    Viewer,
    Renderer
}

#[derive(Debug)]
pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub role: ClientRole,
    pub relay: Box<dyn ClientRelay>
}

impl Client {
    pub fn new<>(name: String, role: ClientRole, relay: Box<dyn ClientRelay>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            role,
            relay
        }
    }
}

pub trait ClientRelay: std::fmt::Debug {
    fn report_client_update(&self, clients: &HashMap<Uuid, Client>);
    fn report_attribute_updates(&self, attributes: &HashMap<AttrName, Attribute>);
    fn report_session_update(&self, state: &SessionState);
}
