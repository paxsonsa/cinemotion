use super::EventBody;
use cinemotion_proto as proto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionOpenedEvent();

impl From<ConnectionOpenedEvent> for EventBody {
    fn from(value: ConnectionOpenedEvent) -> Self {
        Self::ConnectionOpened(value)
    }
}

impl From<ConnectionOpenedEvent> for proto::ConnectionOpenedEvent {
    fn from(_: ConnectionOpenedEvent) -> Self {
        Self {}
    }
}
