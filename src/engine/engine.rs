use std::collections::HashMap;

use derive_more::Constructor;

use crate::api;
use crate::Result;

#[derive(Constructor, Default)]
pub struct Engine {
    pub clients: HashMap<u32, api::models::Client>,
    pub scene: api::models::SceneGraph,
}

impl Engine {
    pub async fn apply(&mut self, command: api::Command) -> Result<()> {
        match command {
            api::Command::Empty => {}
            api::Command::SetClient(client) => {
                match self.clients.get_mut(&client.id) {
                    Some(cur) => {
                        // TODO Handle error capture.
                        let _ = cur.update_from(client);
                    }
                    None => {
                        self.clients.insert(client.id, client);
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn tick(&self) -> Result<api::state::GlobalState> {
        let state = api::GlobalState {
            clients: self.clients.iter().map(|x| x.1.clone()).collect(),
            scene: self.scene.clone(),
        };

        Ok(state)
    }
}
