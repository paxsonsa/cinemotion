use std::fmt::{self, Display, Formatter};

use super::ClientCommand;
use cinemotion_proto::Echo as EchoProto;

#[derive(Debug, Clone)]
pub struct Echo(String);

impl Display for Echo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Echo({})", self.0)
    }
}

/// Create a echo command from a string
impl From<String> for Echo {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Echo> for ClientCommand {
    fn from(value: Echo) -> Self {
        Self::Echo(value)
    }
}

impl From<EchoProto> for Echo {
    fn from(value: EchoProto) -> Self {
        Self(value.message)
    }
}

impl Into<EchoProto> for Echo {
    fn into(self) -> EchoProto {
        EchoProto { message: self.0 }
    }
}

/// Create echo event from echo command
pub struct EchoEvent(String);

impl From<Echo> for EchoEvent {
    fn from(value: Echo) -> Self {
        Self(value.0)
    }
}
