use derive_more::Constructor;

use crate::api;
use crate::{Error, Result};

#[derive(Constructor, Default)]
pub struct Engine {
    pub active_state: api::state::GlobalState,
    pub previous_state: api::state::GlobalState,
}

impl Engine {
    pub async fn apply(&mut self, command: api::Command) -> Result<()> {
        match command {
            api::Command::Empty => {}
            api::Command::SetClient(client) => {
                match self.active_state.clients.get_mut(&client.id) {
                    Some(cur) => {
                        // TODO Handle error capture.
                        let _ = cur.update_from(client);
                    }
                    None => {
                        self.active_state
                            .clients
                            .insert(client.id, api::state::ClientState::from(client));
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn tick(&mut self) -> Result<api::state::GlobalState> {
        self.previous_state = std::mem::take(&mut self.active_state);
        Ok(self.previous_state.clone())
    }
}
