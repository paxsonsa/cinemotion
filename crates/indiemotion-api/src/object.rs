use serde_derive::{Deserialize, Serialize};

///  The object is a wrapper around all possible objects that can be represented in the API.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "lowercase")]
pub enum Object {
    Echo(Echo),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Echo {
    pub message: String
}