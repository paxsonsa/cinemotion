use super::EventBody;
use cinemotion_proto as proto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionOpened {}

impl From<ConnectionOpened> for EventBody {
    fn from(value: ConnectionOpened) -> Self {
        Self::ConnectionOpened(value)
    }
}

impl From<ConnectionOpened> for proto::ConnectionOpenedEvent {
    fn from(_: ConnectionOpened) -> Self {
        Self {}
    }
}
