use serde_derive::{Deserialize, Serialize};

use crate::models::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "lowercase")]
pub enum Command {
    Empty,
    SetClient(Client),
    //
}
