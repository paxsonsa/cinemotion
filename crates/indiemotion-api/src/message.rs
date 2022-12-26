use serde_derive::{Deserialize, Serialize};
use crate::Object;

#[cfg(test)]
#[path = "./message_test.rs"]
mod message_test;

/// Defines the api version for the type of object being loaded
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "apiVersion")]
pub enum ApiVersion {
    #[serde(rename = "v1")]
    V1(Message),
}

impl ApiVersion {
    /// Convert the old api data into the latest format, as needed
    pub fn into_latest(self) -> Message {
        match self {
            Self::V1(o) => o,
        }
    }
}

impl From<Message> for ApiVersion {
    fn from(source: Message) -> Self {
        Self::V1(source)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub struct Message {
    pub header: Header,
    
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// The client id that the message is originating from.
    pub source: String,

    /// An optional destination for the message to be targeting
    pub destination: Option<String>,

    /// The source client time in UTC milliseconds since epoch
    pub source_time_ms: u64,
}

/// A message is the main data unit of the API
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "payload")]
pub enum Payload {
    Single(Object),
    Multi(Vec<Object>),
}
