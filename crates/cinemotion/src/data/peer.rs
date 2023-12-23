use crate::Name;
use cinemotion_proto as proto;
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerRole {
    Controller,
}

impl From<proto::PeerRole> for PeerRole {
    fn from(value: proto::PeerRole) -> Self {
        match value {
            proto::PeerRole::Controller => Self::Controller,
        }
    }
}

impl From<PeerRole> for proto::PeerRole {
    fn from(value: PeerRole) -> Self {
        match value {
            PeerRole::Controller => Self::Controller,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Peer {
    pub uid: u32,
    pub name: Name,
    pub role: PeerRole,
    pub properties: HashMap<Name, String>,
}

impl From<proto::Peer> for Peer {
    fn from(value: proto::Peer) -> Self {
        let role: PeerRole = value.role().into();
        Self {
            uid: value.uid,
            name: value.name.into(),
            role,
            properties: HashMap::new(),
        }
    }
}

impl From<Peer> for proto::Peer {
    fn from(value: Peer) -> Self {
        let role: proto::PeerRole = value.role.into();
        Self {
            uid: value.uid,
            name: value.name.to_string(),
            role: role as i32,
        }
    }
}
