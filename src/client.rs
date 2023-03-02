use std::collections::HashMap;

use uuid::Uuid;

use crate::api;
use crate::runtime::MotionRuntimeObserver;

#[derive(Debug, Default)]
pub struct ClientManager {
    clients: HashMap<Uuid, api::Client>,
}

impl ClientManager {
    pub fn add(&mut self, client: api::Client) {
        self.clients.insert(client.meta.id, client);
    }

    pub fn remove(&mut self, id: Uuid) {
        self.clients.remove(&id);
    }

    pub fn get(&self, id: Uuid) -> Option<api::ClientMetadata> {
        self.clients.get(&id).map(|c| c.meta.clone())
    }
}

#[async_trait::async_trait]
impl MotionRuntimeObserver for ClientManager {
    /// Called when the client list is updated.
    async fn visit_client_update(&self, clients: &Vec<api::ClientMetadata>) {
        todo!()
    }

    /// Called when the session state is updated.
    async fn visit_session_update(&self, state: &api::SessionState) {
        todo!()
    }

    /// Called when a property is updated.
    async fn visit_property_update(&self, properties: &Vec<api::Property>) {
        todo!()
    }
}
