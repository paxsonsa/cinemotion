#[derive(Debug)]
pub struct Context {}

#[derive(Debug, Clone)]
pub enum ContextUpdate {
    Client,
    Session,
    Property,
    Trigger,
}

pub struct ContextChannel {
    channel: tokio::sync::broadcast::Receiver<ContextUpdate>,
}
