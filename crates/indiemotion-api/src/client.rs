use crate::async_trait;
use crate::Error;
use indiemotion_proto as proto;
use std::convert::Into;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ClientRole {
    PrimaryController,
    SecondaryController,
    Observer,
    Renderer,
}
impl Default for ClientRole {
    fn default() -> Self {
        Self::PrimaryController
    }
}

impl From<proto::ClientRole> for ClientRole {
    fn from(value: proto::ClientRole) -> Self {
        match value {
            proto::ClientRole::PrimaryController => Self::PrimaryController,
            proto::ClientRole::SecondaryController => Self::SecondaryController,
            proto::ClientRole::Observer => Self::Observer,
            proto::ClientRole::Renderer => Self::Renderer,
        }
    }
}

impl From<i32> for ClientRole {
    fn from(value: i32) -> Self {
        proto::ClientRole::from_i32(value).unwrap().into()
    }
}

impl TryFrom<String> for ClientRole {
    type Error = Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_str() {
            "primary-controller" | "primary" => Ok(Self::PrimaryController),
            "secondary-controller" | "secondary" => Ok(Self::SecondaryController),
            "observer" => Ok(Self::Observer),
            "renderer" => Ok(Self::Renderer),
            _ => Err(Error::InvalidClientRole(value.to_string())),
        }
    }
}

impl Into<String> for ClientRole {
    fn into(self) -> String {
        match self {
            Self::PrimaryController => "primary-controller".to_string(),
            Self::SecondaryController => "secondary-controller".to_string(),
            Self::Observer => "observer".to_string(),
            Self::Renderer => "renderer".to_string(),
        }
    }
}

impl Into<proto::ClientRole> for ClientRole {
    fn into(self) -> proto::ClientRole {
        match self {
            Self::PrimaryController => proto::ClientRole::PrimaryController,
            Self::SecondaryController => proto::ClientRole::SecondaryController,
            Self::Observer => proto::ClientRole::Observer,
            Self::Renderer => proto::ClientRole::Renderer,
        }
    }
}

impl Into<i32> for ClientRole {
    fn into(self) -> i32 {
        Into::<proto::ClientRole>::into(self).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientID(Uuid);

impl Default for ClientID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<String> for ClientID {
    fn from(value: String) -> Self {
        Self(Uuid::parse_str(&value).unwrap())
    }
}

impl Display for ClientID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "clientid({})", self.0.to_string())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClientMetadata {
    pub id: ClientID,
    pub name: String,
    pub role: ClientRole,
}

impl ClientMetadata {
    /// Create a new ClientMetadata instance with random ID
    pub fn new(name: String, role: ClientRole) -> Self {
        Self {
            id: ClientID(Uuid::new_v4()),
            name,
            role,
        }
    }
}

impl From<proto::ClientInfo> for ClientMetadata {
    fn from(info: proto::ClientInfo) -> Self {
        Self {
            id: info.id.into(),
            name: info.name,
            role: info.role.into(),
        }
    }
}

impl Into<proto::ClientInfo> for ClientMetadata {
    fn into(self) -> proto::ClientInfo {
        proto::ClientInfo {
            id: self.id.0.to_string(),
            name: self.name,
            role: self.role.into(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Client {
    pub meta: ClientMetadata,
    pub relay: Option<Box<dyn ClientRelay>>,
}

impl Client {
    pub fn new(name: String, role: ClientRole) -> Self {
        Self {
            meta: ClientMetadata::new(name, role),
            relay: None,
        }
    }

    pub fn with_relay(mut self, relay: Box<dyn ClientRelay>) -> Self {
        self.relay = Some(relay);
        self
    }
}

#[async_trait]
pub trait ClientRelay: std::fmt::Debug + Send + Sync {}
