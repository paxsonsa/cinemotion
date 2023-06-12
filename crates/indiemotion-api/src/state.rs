use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub struct GlobalState {
    pub clients: HashMap<u32, ClientState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientState {
    pub id: u32,
    pub name: String,
}

impl From<models::Client> for ClientState {
    fn from(client: models::Client) -> Self {
        Self {
            id: client.id,
            name: client.name,
        }
    }
}

impl ClientState {
    pub fn update_from(&mut self, client: models::Client) -> crate::Result<()> {
        self.name = client.name;
        Ok(())
    }
}
