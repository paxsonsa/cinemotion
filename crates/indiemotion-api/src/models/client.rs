use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

#[derive(Constructor, Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: u32,
    pub name: String,
}
