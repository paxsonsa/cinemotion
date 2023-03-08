use std::collections::HashMap;

use uuid::Uuid;

use crate::{api, runtime};

struct ClientChannel {
    channel: runtime::ContextChannel,
}

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
