use serde_derive::{Deserialize, Serialize};

use crate::metadata::HasMetadata;

use super::{Tick, Tock};

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

/// A message is the main data unit of the API
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum Message {
    Tick(Tick),
    Tock(Tock)
}

impl HasMetadata for Message {
    fn metadata(&self) -> crate::metadata::Metadata {
        match self {
            Self::Tick(t) => t.metadata(),
            Self::Tock(t) => t.metadata(),
        }
    }

    fn metadata_mut(&mut self) -> &mut crate::metadata::Metadata {
        match self {
            Self::Tick(t) => t.metadata_mut(),
            Self::Tock(t) => t.metadata_mut(),
        }
    }
}

impl From<Tick> for Message {
    fn from(source: Tick) -> Self {
        Self::Tick(source)
    }
}

impl From<Tock> for Message {
    fn from(source: Tock) -> Self {
        Self::Tock(source)
    }
}