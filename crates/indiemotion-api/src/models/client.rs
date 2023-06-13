use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

#[derive(Constructor, Clone, Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: u32,
    pub name: String,
}

impl Client {
    pub fn update_from(&mut self, client: Client) -> crate::Result<()> {
        self.name = client.name;
        Ok(())
    }
}
