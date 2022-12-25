use serde_derive::{Deserialize, Serialize};

use super::{Metadata, HasMetadata};

/// A request to the server to send its latest tick time
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Tick {
    pub metadata: Metadata,
    pub spec: TickSpec,
}

impl HasMetadata for Tick {
    fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TickSpec {}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Tock {
    pub metadata: Metadata,
    pub spec: TockSpec,
}

impl HasMetadata for Tock {
    fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TockSpec {
    /// The time in UTC milliseconds since epoch that was sent to the
    /// server.
    pub time: u64,
}