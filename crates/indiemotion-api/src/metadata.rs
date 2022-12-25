use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Metadata {
    /// The client id that the message is originating from.
    pub source: String,

    /// An optional destination for the message to be targeting
    pub destination: Option<String>,

    /// The source client time in UTC milliseconds since epoch
    pub source_time_ms: u64,
}

pub trait HasMetadata {
    fn metadata(&self) -> Metadata;
    fn metadata_mut(&mut self) -> &mut Metadata;
}