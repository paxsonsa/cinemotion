use crate::{api, proto};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Context {
    pub clients: HashMap<api::ClientID, api::ClientMetadata>,
    pub entity_count: usize,
    pub entities: Vec<api::Entity>,
}

#[derive(Debug, Clone)]
pub enum ContextUpdate {
    Client(Vec<api::ClientMetadata>),
    Entity(api::Entity),
}

impl From<ContextUpdate> for proto::ContextUpdate {
    fn from(value: ContextUpdate) -> proto::ContextUpdate {
        match value {
            ContextUpdate::Client(clients) => proto::ContextUpdate {
                update_kind: Some(proto::context_update::UpdateKind::Client(
                    proto::ClientUpdate {
                        clients: clients.iter().map(|c| c.clone().into()).collect(),
                    },
                )),
            },
            ContextUpdate::Entity(_) => todo!(),
        }
    }
}
