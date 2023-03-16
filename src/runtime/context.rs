use crate::{api, proto};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Context {
    pub clients: HashMap<api::ClientID, api::ClientMetadata>,
    pub entity_count: usize,
    pub entities: Vec<api::Entity>,
}

#[derive(Debug, Clone)]
pub enum Event {
    Context(Context),
}

impl From<Event> for proto::Event {
    fn from(value: Event) -> proto::Event {
        match value {
            Event::Context(ctx) => {
                let mut context = proto::Context::default();
                context.clients = ctx.clients.values().map(|client| client.into()).collect();
                context.entities = ctx.entities.iter().map(|e| e.into()).collect();
                proto::Event {
                    event: Some(proto::event::Event::ContextUpdate(context)),
                }
            }
        }
    }
}
