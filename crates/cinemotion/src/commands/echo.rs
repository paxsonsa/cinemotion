use super::Command;

pub struct Echo(String);

/// Create a echo command from a string
impl From<String> for Echo {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Echo> for Command {
    fn from(value: Echo) -> Self {
        Self::Echo(value)
    }
}

/// Create echo event from echo command
pub struct EchoEvent(String);

impl From<Echo> for EchoEvent {
    fn from(value: Echo) -> Self {
        Self(value.0)
    }
}
