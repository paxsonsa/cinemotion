use crate::async_trait;
use crate::Error;
use indiemotion_proto as proto;
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

#[derive(Debug, Clone, Default)]
pub struct ClientMetadata {
    pub id: Uuid,
    pub name: String,
    pub role: ClientRole,
}

impl ClientMetadata {
    /// Create a new ClientMetadata instance with random ID
    pub fn new(name: String, role: ClientRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            role,
        }
    }
}

impl From<proto::ClientInfo> for ClientMetadata {
    fn from(info: proto::ClientInfo) -> Self {
        Self {
            id: Uuid::parse_str(&info.id).unwrap(),
            name: info.name,
            role: info.role.into(),
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
