use crate::{api, proto};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
    pub clients: HashMap<api::ClientID, api::ClientMetadata>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContextUpdate {
    Client(Vec<api::ClientMetadata>),
    // Session,
    // Property,
    // Trigger,
    // Ping,
}

impl Into<proto::ContextUpdate> for ContextUpdate {
    fn into(self) -> proto::ContextUpdate {
        match self {
            Self::Client(clients) => proto::ContextUpdate {
                update_kind: Some(proto::context_update::UpdateKind::Client(
                    proto::ClientUpdate {
                        clients: clients.iter().map(|c| c.clone().into()).collect(),
                    },
                )),
            },
        }
    }
}
