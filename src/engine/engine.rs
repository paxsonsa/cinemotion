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
    pub async fn apply(&mut self, command: super::ClientCommand) -> Result<()> {
        let client = command.client;
        let command = command.command;

        match command {
            api::Command::Empty => {}

            api::Command::SceneObject(object) => {
                self.scene.add_object(object).await?;
            }

            api::Command::SetClient(client) => match self.clients.get_mut(&client.id) {
                Some(cur) => {
                    let _ = cur.update_from(client);
                }
                None => {
                    self.clients.insert(client.id, client);
                }
            },
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
