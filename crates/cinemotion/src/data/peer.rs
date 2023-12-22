use cinemotion_proto as proto;
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
    pub name: String,
    pub role: PeerRole,
}

impl From<proto::Peer> for Peer {
    fn from(value: proto::Peer) -> Self {
        let role: PeerRole = value.role().into();
        Self {
            name: value.name,
            role,
        }
    }
}

impl From<Peer> for proto::Peer {
    fn from(value: Peer) -> Self {
        let role: proto::PeerRole = value.role.into();
        Self {
            name: value.name,
            role: role as i32,
        }
    }
}
